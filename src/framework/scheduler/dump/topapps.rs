use std::{
    process::Command,
    sync::atomic::Ordering,
    time::{Duration, Instant},
};

use crate::{defs::RESET_TIME, framework::scheduler::BINDER};
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
     * 使用dumpsys activity lur获取前台
     * 2025-05-24
     */
    pub fn dumpsys(&mut self) {
        if self.time.elapsed() > RESET_TIME {
            let output = loop {
                match Command::new("/system/bin/dumpsys")
                    .args(["activity", "lru"])
                    .output()
                {
                    Ok(o) => {
                        break String::from_utf8_lossy(&o.stdout).to_string();
                    }
                    Err(e) => {
                        log::error!("无法获取前台应用:{e}");
                        std::thread::sleep(Duration::from_secs(1));
                    }
                }
            };

            let re =
                Regex::new(r"  TOP  .*?([a-zA-Z][a-zA-Z0-9_]*(\.[a-zA-Z][a-zA-Z0-9_]*)+)").unwrap();
            for line in output.lines() {
                if let Some(caps) = re.captures(line) {
                    self.topapps = caps[1].to_string();
                    continue;
                }
            }

            self.time = Instant::now();
        }
    }

    /*
     * 使用binder通讯获取
     */
    pub fn binder(&mut self) {
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
        for line in output.lines() {
            if let Some(caps) = re.captures(line) {
                self.topapps = caps[1].to_string();
                continue;
            }
        }

        self.time = Instant::now();
    }

    pub fn dump(&mut self) {
        if BINDER.load(Ordering::SeqCst) {
            self.binder();
        } else {
            self.dumpsys();
        }
    }

    pub fn get(&self) -> String {
        self.topapps.clone()
    }
}
