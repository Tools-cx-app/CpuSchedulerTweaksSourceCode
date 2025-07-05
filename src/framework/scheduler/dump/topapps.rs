use std::time::{Duration, Instant};

use crate::defs::RESET_TIME;
use dumpsys_rs::Dumpsys;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct TopAppWatch {
    time: Instant,
    topapps: String,
}

impl TopAppWatch {
    pub fn new() -> Self {
        Self {
            time: Instant::now(),
            topapps: String::new(),
        }
    }

    /*
     * 使用binder通讯获取
     */
    pub fn dump(&mut self) {
        let dumper = loop {
            match Dumpsys::new("activity") {
                Some(s) => break s,
                None => std::thread::sleep(std::time::Duration::from_secs(1)),
            };
        };
        let output = loop {
            match dumper.dump(&["lru"]) {
                Ok(d) => break d,
                Err(e) => {
                    log::error!("无法获取前台应用: {e}");
                    std::thread::sleep(Duration::from_secs(1));
                }
            };
        };

        let re =
            Regex::new(r"  TOP  .*?([a-zA-Z][a-zA-Z0-9_]*(\.[a-zA-Z][a-zA-Z0-9_]*)+)").unwrap();
        let mut count = 0;
        for line in output.lines() {
            if let Some(caps) = re.captures(line) {
                count += 1;
                if count == 1 {
                    continue;
                }
                self.topapps = caps[1].to_string();
                continue;
            }
        }

        self.time = Instant::now();
    }

    pub fn get(&self) -> String {
        self.topapps.clone()
    }
}
