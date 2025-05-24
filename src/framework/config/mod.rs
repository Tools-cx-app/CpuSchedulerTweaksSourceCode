use std::{collections::HashMap, fs};

use data::{CpuCtl, CpuCtlInfo, CpuCtlUclamp, CpuInfo, FreqInfo};

use crate::defs;

pub mod data;

impl data::ConfigData {
    pub fn new() -> Self {
        let s = Self {
            powersave: CpuInfo {
                super_big_cpu_freq: None,
                big_cpu_freq: FreqInfo { max: 0, min: 0 },
                middle_cpu_freq: FreqInfo { max: 0, min: 0 },
                small_cpu_freq: None,
                super_big_cpu_governor: None,
                big_cpu_governor: String::new(),
                middle_cpu_governor: String::new(),
                small_cpu_governor: None,
                cpuctl: CpuCtl {
                    top_app: CpuCtlInfo {
                        shares: 0,
                        uclamp: CpuCtlUclamp { max: 0, min: 0 },
                    },
                    foreground: CpuCtlInfo {
                        shares: 0,
                        uclamp: CpuCtlUclamp { max: 0, min: 0 },
                    },
                },
            },
            balance: CpuInfo {
                super_big_cpu_freq: None,
                big_cpu_freq: FreqInfo { max: 0, min: 0 },
                middle_cpu_freq: FreqInfo { max: 0, min: 0 },
                small_cpu_freq: None,
                super_big_cpu_governor: None,
                big_cpu_governor: String::new(),
                middle_cpu_governor: String::new(),
                small_cpu_governor: None,
                cpuctl: CpuCtl {
                    top_app: CpuCtlInfo {
                        shares: 0,
                        uclamp: CpuCtlUclamp { max: 0, min: 0 },
                    },
                    foreground: CpuCtlInfo {
                        shares: 0,
                        uclamp: CpuCtlUclamp { max: 0, min: 0 },
                    },
                },
            },
            performance: CpuInfo {
                super_big_cpu_freq: None,
                big_cpu_freq: FreqInfo { max: 0, min: 0 },
                middle_cpu_freq: FreqInfo { max: 0, min: 0 },
                small_cpu_freq: None,
                super_big_cpu_governor: None,
                big_cpu_governor: String::new(),
                middle_cpu_governor: String::new(),
                small_cpu_governor: None,
                cpuctl: CpuCtl {
                    top_app: CpuCtlInfo {
                        shares: 0,
                        uclamp: CpuCtlUclamp { max: 0, min: 0 },
                    },
                    foreground: CpuCtlInfo {
                        shares: 0,
                        uclamp: CpuCtlUclamp { max: 0, min: 0 },
                    },
                },
            },
            fast: CpuInfo {
                super_big_cpu_freq: None,
                big_cpu_freq: FreqInfo { max: 0, min: 0 },
                middle_cpu_freq: FreqInfo { max: 0, min: 0 },
                small_cpu_freq: None,
                super_big_cpu_governor: None,
                big_cpu_governor: String::new(),
                middle_cpu_governor: String::new(),
                small_cpu_governor: None,
                cpuctl: CpuCtl {
                    top_app: CpuCtlInfo {
                        shares: 0,
                        uclamp: CpuCtlUclamp { max: 0, min: 0 },
                    },
                    foreground: CpuCtlInfo {
                        shares: 0,
                        uclamp: CpuCtlUclamp { max: 0, min: 0 },
                    },
                },
            },
            cpu_config: data::Cpu {
                super_big: None,
                big: 0,
                middle: 0,
                small: None,
            },
            osm: String::new(),
            applist: HashMap::new(),
        };
        s.load_config()
    }

    pub fn load_config(&self) -> Self {
        if let Err(e) = fs::read_to_string(defs::CONFIG_PATH) {
            log::error!("无法读取配置文件:{},请检查{}", e, defs::CONFIG_PATH);
        }
        let content = fs::read_to_string(defs::CONFIG_PATH).unwrap();
        let toml: Self = toml::from_str(content.as_str()).unwrap();
        toml
    }
}
