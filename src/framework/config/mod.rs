use std::fs;

use crate::defs;

pub mod data;

impl data::ConfigData {
    pub fn new() -> Self {
        load_config()
    }

    pub fn load_config(&self) -> Self {
        load_config()
    }

    pub fn is_emtpy(&self) -> bool {
        self.osm.is_empty()
    }
}

pub fn load_config() -> data::ConfigData {
    if let Err(e) = fs::read_to_string(defs::CONFIG_PATH) {
        log::error!("无法读取配置文件:{},请检查{}", e, defs::CONFIG_PATH);
    }
    let content = fs::read_to_string(defs::CONFIG_PATH).unwrap();
    let toml: data::ConfigData = match toml::from_str(content.as_str()) {
        Ok(s) => s,
        Err(e) => {
            log::error!("配置文件错误:{}", e);
            std::process::exit(2);
        }
    };
    toml
}
