mod defs;
mod framework;
mod utils;

use std::{
    fs::{self, OpenOptions, create_dir_all},
    io::{Read, Write},
    os::raw::c_void,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context, Result, anyhow};
use defs::MOD_PROP_PATH;
use env_logger::Builder;
use libc::{c_int, fork, kill, setsid, umask};
use regex::Regex;

use crate::{
    defs::{GET_KERNELSU_VERSION, KERNEL_SU_OPTION, RESET_TIME},
    utils::files::write_with_locked,
};

fn check() -> Result<()> {
    let procs = procfs::process::all_processes().context("无法获取进程列表")?;
    let self_name = fs::read_to_string("/proc/self/comm")?;

    for proc in procs {
        let proc = match proc {
            Ok(p) => p,
            Err(e) => {
                log::error!("无法获取进程信息:{}", e);
                break;
            }
        };
        if let Ok(states) = proc.status() {
            if states.name.contains(self_name.trim()) && states.pid != std::process::id() as i32 {
                log::error!("发现其他进程,pid:{},即将退出", states.pid);
                std::process::exit(1);
            }
        }
    }
    Ok(())
}

fn init_logger() -> Result<()> {
    let mut builder = Builder::new();

    builder.format(|buf, record| {
        let local_time = chrono::Local::now();
        let time_str = local_time.format("%Y-%m-%d %H:%M:%S%.3f").to_string();

        writeln!(buf, "[{}] [{}] {}", time_str, record.level(), record.args())
    });
    builder.filter_level(log::LevelFilter::Info).init();
    Ok(())
}

fn daemon() -> Result<()> {
    unsafe {
        match fork() {
            -1 => return Err(anyhow!("fork进程失败")),
            0 => {}
            _ => std::process::exit(0),
        }

        if setsid() == -1 {
            return Err(anyhow!("setsid失败"));
        }

        match fork() {
            -1 => return Err(anyhow!("fork进程失败")),
            0 => {}
            _ => std::process::exit(0),
        }

        umask(0);
    }
    Ok(())
}

fn check_pid(pid: i32, sig: i32) -> bool {
    unsafe { kill(pid, sig) != 0 }
}

fn create_daemon() {
    let pid = std::process::id() as i32;

    match unsafe { fork() } {
        -1 => {
            log::error!("fork失败");
            std::process::exit(-2);
        }
        0 => {
            if let Err(e) = daemon() {
                log::error!("daemon启动失败:{}", e);
                std::process::exit(-3);
            }
            loop {
                if check_pid(pid, 0) {
                    // 每次写入前重新打开文件（覆盖模式）
                    /*let mut = OpenOptions::new()
                    .write(true)
                    .truncate(true) // 关键修改：清空文件
                    .open(MOD_PROP_PATH)
                    .unwrap();*/

                    let mut buf = String::new();
                    // 重新打开文件读取内容（与写入分开）
                    std::fs::File::open(MOD_PROP_PATH)
                        .unwrap()
                        .read_to_string(&mut buf)
                        .unwrap();

                    let re = Regex::new(r"(?m)^(description\s*=\s*).*$").unwrap();
                    let new_content = re.replace_all(&buf, |caps: &regex::Captures| {
                        format!(
                            "{}{}",
                            &caps[1], "[运行状态: 未运行 🥵🥵] 适用于大部分设备的CPU动态调速器"
                        )
                    });

                    fs::write(MOD_PROP_PATH, new_content.as_bytes()).unwrap();
                    std::process::exit(-4);
                } else {
                    // 每次写入前重新打开文件（覆盖模式）
                    /* let mut file = OpenOptions::new()
                    .write(true)
                    .truncate(true) // 关键修改：清空文件
                    .open(MOD_PROP_PATH)
                    .unwrap();*/

                    let mut buf = String::new();
                    // 重新打开文件读取内容（与写入分开）
                    std::fs::File::open(MOD_PROP_PATH)
                        .unwrap()
                        .read_to_string(&mut buf)
                        .unwrap();

                    let re = Regex::new(r"(?m)^(description\s*=\s*).*$").unwrap();
                    let new_content = re.replace_all(&buf, |caps: &regex::Captures| {
                        format!(
                            "{}{}",
                            &caps[1], "[运行状态: 运行中 😋😋] 适用于大部分设备的CPU动态调速器"
                        )
                    });

                    fs::write(MOD_PROP_PATH, new_content.as_bytes()).unwrap();
                }
                std::thread::sleep(RESET_TIME);
            }
        }
        _ => {}
    }
}

fn ksuctl(cmd: c_int, arg1: *mut c_void, arg2: *mut c_void) -> bool {
    unsafe {
        let mut result: u32 = 0;
        // 调用 prctl 系统调用
        libc::syscall(
            libc::SYS_prctl,
            KERNEL_SU_OPTION as libc::c_ulong,
            cmd as libc::c_ulong,
            arg1 as libc::c_ulong,
            arg2 as libc::c_ulong,
            &mut result as *mut u32 as libc::c_ulong,
        );
        result == KERNEL_SU_OPTION
    }
}

fn get_kernelsu_version() -> c_int {
    let mut version: c_int = -1;
    ksuctl(
        GET_KERNELSU_VERSION,
        &mut version as *mut c_int as *mut c_void,
        std::ptr::null_mut(),
    );
    version
}

fn main() -> Result<()> {
    init_logger().context("初始化日志加载器失败")?;
    check()?;
    create_daemon();
    log::info!("CpuSchedulerTweaks v{}", defs::VERSION);
    let ksu_version = get_kernelsu_version();
    let magisk_version = match Command::new("magisk").arg("-V").output() {
        Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
        Err(_) => String::new(),
    };
    if ksu_version != -1 {
        log::info!("KernelSU Kernel版本:{}", ksu_version);
    }
    if !magisk_version.is_empty() {
        log::info!("Magisk版本:{}", magisk_version);
    }
    let mut framework = crate::framework::scheduler::Looper::new();
    framework.init();
    framework.enter_looper()?;
    Ok(())
}
