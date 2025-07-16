pub mod freq;
pub mod governor;

use std::{path::Path, thread};

use anyhow::Result;

use crate::{
    framework::{config::data::ConfigData, scheduler::AUTO},
    utils::{cpu::CpuLoadUtils, files::write_with_locked},
};

pub struct Cpu {
    config: ConfigData,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            config: ConfigData::new(),
        }
    }

    /*
     * 配置文件加载地方
     * 2025-05-24
     */
    pub fn load_config(&mut self, config: ConfigData) {
        self.config = config;
    }

    pub fn get_cluster_paths(&self) -> ClusterPaths {
        ClusterPaths {
            big: self
                .config
                .cpu_config
                .big
                .map(|id| format!("/sys/devices/system/cpu/cpufreq/policy{id}")),
            middle: self
                .config
                .cpu_config
                .middle
                .map(|id| format!("/sys/devices/system/cpu/cpufreq/policy{id}")),
            small: self
                .config
                .cpu_config
                .small
                .map(|id| format!("/sys/devices/system/cpu/cpufreq/policy{id}")),
            super_big: self
                .config
                .cpu_config
                .super_big
                .map(|id| format!("/sys/devices/system/cpu/cpufreq/policy{id}")),
        }
    }
}

pub struct ClusterPaths {
    pub big: Option<String>,
    pub middle: Option<String>,
    pub small: Option<String>,
    pub super_big: Option<String>,
}

impl ClusterPaths {
    // 检查路径存在性
    pub fn check_existence(&self) -> (bool, bool, bool, bool) {
        (
            self.big
                .as_ref()
                .map(|p| Path::new(p).exists())
                .unwrap_or(false),
            self.middle
                .as_ref()
                .map(|p| Path::new(p).exists())
                .unwrap_or(false),
            self.small
                .as_ref()
                .map(|p| Path::new(p).exists())
                .unwrap_or(false),
            self.super_big
                .as_ref()
                .map(|p| Path::new(p).exists())
                .unwrap_or(false),
        )
    }
}

pub fn auto_load() -> Result<()> {
    let mut cpu = CpuLoadUtils::new()?;

    if AUTO.load(std::sync::atomic::Ordering::Relaxed) {
        thread::spawn(move || -> Result<()> {
            loop {
                for (id, load) in cpu.get_cpu_load()? {
                    let path = Path::new(format!("/sys/devices/system/cpu/cpu{}", id).as_str());
                    log::debug!("core{} load is {}", id, load);

                    if load > 90.0 {
                        write_with_locked(path, "9999999");
                    }

                    if load > 60.0 && load < 90.0 {
                        write_with_locked(path, "2000000");
                    }
                }
            }
        });
    }
    Ok(())
}
