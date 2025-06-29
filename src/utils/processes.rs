use std::io;

use anyhow::{Context, Result, anyhow};
use libc::{PRIO_MAX, PRIO_MIN, PRIO_PROCESS, setpriority};

/// 设置进程优先级
/// pid: 目标id
/// level: 目标优先级 (-20 到 19)
pub fn set_current_priority(pid: u32, level: i32) -> Result<()> {
    // 验证优先级范围
    if level < PRIO_MIN || level > PRIO_MAX {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("优先级必须介于 {} 和 {} 之间", PRIO_MIN, PRIO_MAX),
        )
        .into());
    }

    #[cfg(target_os = "android")]
    let result = unsafe { setpriority(PRIO_PROCESS, pid, level) };

    #[cfg(target_os = "linux")]
    let result = unsafe { setpriority(PRIO_PROCESS, pid, level) };

    if result == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error().into())
    }
}

/// 获取进程pid
/// package: 包名
pub fn get_pid(package: &str) -> Result<u32> {
    let procs = procfs::process::all_processes().context("无法获取进程列表")?;

    for proc in procs {
        let proc = match proc {
            Ok(p) => p,
            Err(e) => {
                log::error!("无法获取进程信息:{}", e);
                break;
            }
        };
        if let Ok(states) = proc.status() {
            if states.name.contains(package) {
                return Ok(states.pid as u32);
            }
        }
    }

    Ok(0)
}
