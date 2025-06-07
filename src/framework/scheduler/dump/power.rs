use std::{
    process::Command,
    time::{Duration, Instant},
};

use regex::Regex;

use crate::defs::RESET_TIME;

pub struct Power {
    pub state: bool,
    time: Instant,
}

impl Power {
    pub fn new() -> Self {
        Self {
            time: Instant::now(),
            state: false,
        }
    }

    pub fn dump(&mut self) {
        if self.time.elapsed() > RESET_TIME {
            let output = loop {
                match Command::new("/system/bin/dumpsys").args(["power"]).output() {
                    Ok(o) => {
                        break String::from_utf8_lossy(&o.stdout).to_string();
                    }
                    Err(e) => {
                        log::error!("无法获取前台应用:{e}");
                        std::thread::sleep(Duration::from_secs(1));
                    }
                }
            };

            let patterns = [
                r"Display Power: state=ON",
                r"mScreenOn=true",
                r"mWakefulness=Awake",
                r"mWakefulness=Full",
            ];

            self.state = patterns
                .iter()
                .any(|&pattern| Regex::new(pattern).unwrap().is_match(output.as_str()));
        }
    }
}
