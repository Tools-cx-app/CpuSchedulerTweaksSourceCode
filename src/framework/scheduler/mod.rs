mod cpu;
mod cpuctl;
mod dump;

use std::{
    fs::write,
    process::Command,
    sync::atomic::{AtomicBool, Ordering},
};

use anyhow::Result;
use cpu::{Cpu, freq::CpuFreqs, governor::CpuGovernor};
use cpuctl::CpuCtl;
use dump::topapps::TopAppWatch;
use glob::glob;
use inotify::{Inotify, WatchMask};

use super::config::data::ConfigData;
use crate::defs;

static BINDER: AtomicBool = AtomicBool::new(false);
static DEBUG: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Powersave,
    Balance,
    Performance,
    Fast,
}

pub struct Looper {
    cpu: Cpu,
    cpuctl: CpuCtl,
    mode: Mode,
    config: ConfigData,
    topapp: TopAppWatch,
}

impl Looper {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            cpuctl: CpuCtl::new(),
            mode: Mode::Balance,
            config: ConfigData::new(),
            topapp: TopAppWatch::new(),
        }
    }

    /*
     * 初始化部分，用于禁用一些系统东西
     * 2025-05-24
     */
    pub fn init(&self) {
        let commands = vec![
            ("stop", vec!["miuibooster"]),
            ("stop", vec!["oneplus_brain_service"]),
            ("stop", vec!["vendor.perfservice"]),
            ("stop", vec!["perfd"]),
            (
                "stop",
                vec!["orms-hal-1-0", "vendor.oplus.ormsHalService-aidl-default"],
            ),
            ("resetprop", vec!["persist.sys.hardcoder.name", ""]),
            ("resetprop", vec!["persist.miui.miperf.enable", "false"]),
        ];

        for i in defs::BOOST_PATHS {
            for path in glob(i).unwrap() {
                let path = path.unwrap();
                let _ = write(&path, "0");
                log::debug!("{}", path.display());
            }
        }
        let _ = write(
            "/sys/devices/system/cpu/cpufreq/hotplug/cpu_hotplug_disable",
            "1",
        );
        for path in glob("/sys/module/control_center/parameters/*").unwrap() {
            let path = path.unwrap();
            let _ = write(&path, "N");
            log::debug!("{}", path.display());
        }

        for (command, args) in commands {
            let _ = Command::new(command).args(args).output();
        }
    }

    /*
     * 进入looper
     * 2025-05-31
     */
    pub fn enter_looper(&mut self) -> Result<()> {
        let mut inotify = Inotify::init()?;
        let mut app_cache = Some(String::new());
        inotify.watches().add("/dev/input", WatchMask::ACCESS)?;

        if self.config.binder {
            BINDER.store(true, Ordering::SeqCst);
        }
        if self.config.debug {
            DEBUG.store(true, Ordering::SeqCst);
        }

        loop {
            inotify.read_events_blocking(&mut [0; 1024])?;
            self.cpu.load_config(self.config.clone());
            self.cpuctl.load_config(self.config.clone());
            self.config = self.config.load_config();
            self.topapp.dump();
            
            if DEBUG.load(Ordering::SeqCst) {
                log::debug!("当前topapp: {}", self.topapp.get());
                log::debug!("当前mode: {:?}", self.mode);
                log::debug!("当前config: {:?}", self.config);
            }

            for (app, mode) in self.config.applist.clone() {
                if app_cache.clone().unwrap_or_default() != self.topapp.get()
                    && self.topapp.get() == app
                {
                    log::info!("正在为{app}配置{mode}模式");
                    self.cpu.set_freq(self.switch_mode(mode.as_str()));
                    self.cpu.set_governor(self.switch_mode(mode.as_str()));
                    self.cpuctl.set_uclamp(self.switch_mode(mode.as_str()));
                    app_cache = Some(app);
                } else {
                    self.cpu
                        .set_freq(self.switch_mode(self.config.osm.as_str()));
                    self.cpu
                        .set_governor(self.switch_mode(self.config.osm.as_str()));
                    self.cpuctl
                        .set_uclamp(self.switch_mode(self.config.osm.as_str()));
                }
            }
        }
    }

    fn switch_mode(&self, mode: &str) -> Mode {
        match mode {
            "powersave" => Mode::Powersave,
            "balance" => Mode::Balance,
            "performance" => Mode::Performance,
            "fast" => Mode::Fast,
            _ => {
                log::error!("配置文件错误,进程退出");
                std::process::exit(1);
            }
        }
    }
}
