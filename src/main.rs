mod defs;
mod framework;
mod utils;

use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
};

use anyhow::{Context, Result, anyhow};
use defs::MOD_PROP_PATH;
use env_logger::Builder;
use libc::{STDERR_FILENO, STDIN_FILENO, STDOUT_FILENO, close, fork, kill, setsid, umask};
use regex::Regex;

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
            eprintln!("fork失败");
            std::process::exit(-2);
        }
        0 => {
            if let Err(e) = daemon() {
                log::error!("daemon启动失败");
                std::process::exit(-3);
            }
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .open(MOD_PROP_PATH)
                .unwrap();
            loop {
                if check_pid(pid, 0) {
                    let buf;
                    file.read_to_string(buf);
                    let re = Regex::new(r"(?m)^(description\s*=\s*).*$").unwrap();

                    file.write(
                        re.replace_all(&buf, |caps: &regex::Captures| {
                            format!(
                                "{}{}",
                                &caps[1], "[运行状态: 未运行] 适用于大部分设备的CPU动态调速器"
                            )
                        })
                        .to_string(),
                    );

                    std::process::exit(-4);
                } else {
                    let buf;
                    file.read_to_string(buf);
                    let re = Regex::new(r"(?m)^(description\s*=\s*).*$").unwrap();

                    file.write(
                        re.replace_all(&buf, |caps: &regex::Captures| {
                            format!(
                                "{}{}",
                                &caps[1], "[运行状态: 运行中] 适用于大部分设备的CPU动态调速器"
                            )
                        })
                        .to_string(),
                    );
                }
            }
        }
        _ => {}
    }
}

fn main() -> Result<()> {
    init_logger().context("初始化日志加载器失败")?;
    check()?;
    create_daemon();
    let _ = fs::write(
        "/dev/cpuset/background/cgroup.procs",
        std::process::id().to_string(),
    );
    log::info!("life-death-scheduler v{}", defs::VERSION);
    log::info!("life-death-scheduler正在启动");
    let mut framework = crate::framework::scheduler::Looper::new();
    framework.init();
    framework.enter_looper()?;
    Ok(())
}
