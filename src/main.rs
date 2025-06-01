mod defs;
mod framework;
mod utils;

use std::{fs, io::Write};

use anyhow::{Context, Result};
use env_logger::Builder;

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
    builder.filter_level(log::LevelFilter::Debug).init();
    Ok(())
}
fn main() -> Result<()> {
    init_logger().context("初始化日志加载器失败")?;
    check()?;
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
