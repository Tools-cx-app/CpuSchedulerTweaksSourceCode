use std::{collections::HashMap, fs};

use data::{CpuCtl, CpuCtlInfo, CpuCtlUclamp, CpuFreqs, CpuInfo, FreqInfo, Governor};

use crate::defs;

pub mod data;

impl data::ConfigData {
    pub fn new() -> Self {
        let s = Self {
            powersave: CpuInfo {
                freqs: CpuFreqs {
                    super_big_cpu: None,
                    big_cpu: None,
                    middle_cpu: None,
                    small_cpu: None,
                },
                governor: Governor {
                    super_big_cpu: None,
                    big_cpu: None,
                    middle_cpu: None,
                    small_cpu: None,
                },
                cpuctl: CpuCtl {
                    top_app: CpuCtlInfo {
                        shares: 0,
                        uclamp: CpuCtlUclamp { max: 0, min: 0 },
                    },
                    /*foreground: CpuCtlInfo {
                        shares: 0,
                        uclamp: CpuCtlUclamp { max: 0, min: 0 },
                    },*/
                },
            },
            balance: CpuInfo {
                freqs: CpuFreqs {
                    super_big_cpu: None,
                    big_cpu: None,
                    middle_cpu: None,
                    small_cpu: None,
                },
                governor: Governor {
                    super_big_cpu: None,
                    big_cpu: None,
                    middle_cpu: None,
                    small_cpu: None,
                },
                cpuctl: CpuCtl {
                    top_app: CpuCtlInfo {
                        shares: 0,
                        uclamp: CpuCtlUclamp { max: 0, min: 0 },
                    },
                    /*foreground: CpuCtlInfo {
                        shares: 0,
                        uclamp: CpuCtlUclamp { max: 0, min: 0 },
                    },*/
                },
            },
            performance: CpuInfo {
                freqs: CpuFreqs {
                    super_big_cpu: None,
                    big_cpu: None,
                    middle_cpu: None,
                    small_cpu: None,
                },
                governor: Governor {
                    super_big_cpu: None,
                    big_cpu: None,
                    middle_cpu: None,
                    small_cpu: None,
                },
                cpuctl: CpuCtl {
                    top_app: CpuCtlInfo {
                        shares: 0,
                        uclamp: CpuCtlUclamp { max: 0, min: 0 },
                    },
                    /*foreground: CpuCtlInfo {
                        shares: 0,
                        uclamp: CpuCtlUclamp { max: 0, min: 0 },
                    },*/
                },
            },
            fast: CpuInfo {
                freqs: CpuFreqs {
                    super_big_cpu: None,
                    big_cpu: None,
                    middle_cpu: None,
                    small_cpu: None,
                },
                governor: Governor {
                    super_big_cpu: None,
                    big_cpu: None,
                    middle_cpu: None,
                    small_cpu: None,
                },
                cpuctl: CpuCtl {
                    top_app: CpuCtlInfo {
                        shares: 0,
                        uclamp: CpuCtlUclamp { max: 0, min: 0 },
                    },
                    /*foreground: CpuCtlInfo {
                        shares: 0,
                        uclamp: CpuCtlUclamp { max: 0, min: 0 },
                    },*/
                },
            },
            cpu_config: data::Cpu {
                super_big: None,
                big: None,
                middle: None,
                small: None,
            },
            osm: String::new(),
            applist: HashMap::new(),
            binder: false,
            debug: false,
        };
        s.load_config()
    }

    pub fn load_config(&self) -> Self {
        if let Err(e) = fs::read_to_string(defs::CONFIG_PATH) {
            log::error!("无法读取配置文件:{},请检查{}", e, defs::CONFIG_PATH);
        }
        let content = fs::read_to_string(defs::CONFIG_PATH).unwrap();
        let toml: Self = match toml::from_str(content.as_str()) {
            Ok(s) => s,
            Err(e) => {
                log::error!("配置文件错误:{}", e);
                std::process::exit(2);
            }
        };
        toml
    }
}
