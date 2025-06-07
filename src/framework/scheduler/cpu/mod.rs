pub mod freq;
pub mod governor;

use std::path::Path;

use crate::framework::config::data::ConfigData;

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
            big: self.config.cpu_config.big.map(|id| format!("/sys/devices/system/cpu/cpufreq/policy{id}")),
            middle: self.config.cpu_config.middle.map(|id| format!("/sys/devices/system/cpu/cpufreq/policy{id}")),
            small: self.config.cpu_config.small.map(|id| format!("/sys/devices/system/cpu/cpufreq/policy{id}")),
            super_big: self.config.cpu_config.super_big.map(|id| format!("/sys/devices/system/cpu/cpufreq/policy{id}")),
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
            self.big.as_ref().map(|p| Path::new(p).exists()).unwrap_or(false),
            self.middle.as_ref().map(|p| Path::new(p).exists()).unwrap_or(false),
            self.small.as_ref().map(|p| Path::new(p).exists()).unwrap_or(false),
            self.super_big.as_ref().map(|p| Path::new(p).exists()).unwrap_or(false),
        )
    }

    // 调试打印
    pub fn debug_print(&self) {
        if let Some(b) = &self.big {
            log::debug!("big簇: {}", b);
        }
        if let Some(m) = &self.middle {
            log::debug!("middle簇: {}", m);
        }
        if let Some(s) = &self.small {
            log::debug!("small簇: {}", s);
        }
        if let Some(sb) = &self.super_big {
            log::debug!("super_big簇: {}", sb);
        }
    }
}