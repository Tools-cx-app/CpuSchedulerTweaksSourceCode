use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ConfigData {
    pub osm: String,
    pub binder: bool,
    pub debug: bool,
    pub cpu_config: Cpu,
    pub powersave: CpuInfo,
    pub balance: CpuInfo,
    pub performance: CpuInfo,
    pub fast: CpuInfo,
    pub applist: HashMap<String, String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CpuInfo {
    pub super_big_cpu_freq: Option<FreqInfo>,
    pub big_cpu_freq: FreqInfo,
    pub middle_cpu_freq: FreqInfo,
    pub small_cpu_freq: Option<FreqInfo>,
    pub super_big_cpu_governor: Option<String>,
    pub big_cpu_governor: String,
    pub middle_cpu_governor: String,
    pub small_cpu_governor: Option<String>,
    pub cpuctl: CpuCtl,
}

#[derive(Deserialize, Debug, Clone, Copy, Default)]
pub struct FreqInfo {
    pub max: u64,
    pub min: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Cpu {
    pub super_big: Option<u16>,
    pub big: u16,
    pub middle: u16,
    pub small: Option<u16>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CpuCtl {
    pub top_app: CpuCtlInfo,
    pub foreground: CpuCtlInfo,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CpuCtlInfo {
    pub shares: u16,
    pub uclamp: CpuCtlUclamp,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CpuCtlUclamp {
    pub max: u16,
    pub min: u16,
}
