use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ConfigData {
    pub osm: String,
    pub cpu_config: Cpu,
    pub powersave: CpuFreq,
    pub balance: CpuFreq,
    pub performance: CpuFreq,
    pub fast: CpuFreq,
    pub applist: HashMap<String, String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CpuFreq {
    pub super_big_cpu_freq: Option<FreqInfo>,
    pub big_cpu_freq: FreqInfo,
    pub middle_cpu_freq: FreqInfo,
    pub small_cpu_freq: Option<FreqInfo>,
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
