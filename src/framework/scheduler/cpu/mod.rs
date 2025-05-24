pub mod freq;
pub mod governor;

use crate::framework::config::data::{ConfigData, FreqInfo};

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

    /*
     * Option类型转换
     * 2025-05-24
     */
    fn option_to_no(&self, s: Option<FreqInfo>) -> FreqInfo {
        s.unwrap_or_default()
    }
    
    fn option_to_string(&self, s: Option<String>) -> String {
        s.unwrap_or_default()
    }
}