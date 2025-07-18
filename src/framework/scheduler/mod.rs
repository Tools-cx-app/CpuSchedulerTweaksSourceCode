mod cpu;
mod cpuctl;
mod dump;

use std::{
    fmt,
    fs::{self, write},
    path::Path,
    process::Command,
    sync::atomic::AtomicBool,
};

use anyhow::Result;
use cpu::{Cpu, freq::CpuFreqs, governor::CpuGovernor};
use cpuctl::CpuCtl;
use dump::topapps::TopAppWatch;
use glob::glob;
use inotify::{Inotify, WatchMask};

use super::config::data::ConfigData;
use crate::{
    defs::{self, SDC_READ_AHEAD, SDC_SCHEDULER},
    framework::scheduler::{cpu::auto_load, dump::power::Power},
    utils::{
        files::write_with_locked,
        processes::{get_pid, set_current_priority},
    },
};

static AUTO: AtomicBool = AtomicBool::new(false);
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
    power: Power,
    config: ConfigData,
    topapp: TopAppWatch,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Powersave => write!(f, "powersave"),
            Mode::Balance => write!(f, "balance"),
            Mode::Performance => write!(f, "performance"),
            Mode::Fast => write!(f, "fast"),
        }
    }
}
impl Looper {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            cpuctl: CpuCtl::new(),
            power: Power::new(),
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
        let mut config_cache = ConfigData::new();
        inotify.watches().add("/dev/input", WatchMask::ACCESS)?;

        let sdc_scheduler = fs::read_to_string(SDC_SCHEDULER)?;
        if sdc_scheduler.contains(&self.config.io.scheduler) {
            write_with_locked(Path::new(SDC_SCHEDULER), self.config.io.scheduler.as_str())?;
        } else {
            log::warn!("io.scheduler配置项错误");
        }
        write_with_locked(
            Path::new(SDC_READ_AHEAD),
            self.config.io.read_ahead.to_string().as_str(),
        )?;

        self.cpuctl.set_uclamp(Mode::Fast);
        self.cpu.set_governor(Mode::Fast);
        self.cpu.set_freq(Mode::Fast);
        std::thread::sleep(std::time::Duration::from_secs(10));

        if self.config.auto {
            AUTO.store(true, std::sync::atomic::Ordering::Relaxed);
            auto_load()?;
        }

        let surfaceflinger_pid = get_pid("surfaceflinger")?;
        let launcher3_pid = get_pid("com.android.launcher3")?;
        let nexuslauncher_pid = get_pid("com.google.android.apps.nexuslauncher")?;
        let hw_lanucher_pid = get_pid("com.huawei.android.launcher")?;
        let miui_home_pid = get_pid("com.miui.home")?;
        let vivo_launcher_pid = get_pid("com.vivo.launcher")?;
        let oppo_launcher_pid = get_pid("com.oppo.launcher")?;
        let sec_launcher_pid = get_pid("com.sec.android.app.launcher")?;
        set_current_priority(surfaceflinger_pid, -20)?;
        set_current_priority(std::process::id() as u32, 10)?;
        set_current_priority(launcher3_pid, -15)?;
        set_current_priority(nexuslauncher_pid, -15)?;
        set_current_priority(hw_lanucher_pid, -15)?;
        set_current_priority(miui_home_pid, -15)?;
        set_current_priority(vivo_launcher_pid, -15)?;
        set_current_priority(oppo_launcher_pid, -15)?;
        set_current_priority(sec_launcher_pid, -15)?;

        loop {
            inotify.read_events_blocking(&mut [0; 1024])?;
            self.config = self.config.load_config();
            if config_cache.clone().is_emtpy() {
                self.cpu.load_config(self.config.clone());
                self.cpuctl.load_config(self.config.clone());
                config_cache = self.config.clone();
                if self.config.debug {
                    log::set_max_level(log::LevelFilter::Debug);
                    log::info!("日志等级为Debug");
                } else {
                    log::set_max_level(log::LevelFilter::Info);
                    log::info!("日志等级为Info");
                }
            } else if config_cache != self.config.clone() {
                self.cpu.load_config(self.config.clone());
                self.cpuctl.load_config(self.config.clone());
                config_cache = self.config.clone();
                if self.config.debug {
                    log::set_max_level(log::LevelFilter::Debug);
                    log::info!("日志等级为Debug");
                } else {
                    log::set_max_level(log::LevelFilter::Info);
                    log::info!("日志等级为Info");
                }
                log::info!("配置文件已重载");
            }
            self.power.dump();
            self.topapp.dump();

            if !self.power.state {
                if !self.config.auto {
                    self.cpu.set_freq(Mode::Powersave);
                }
                self.cpu.set_governor(Mode::Powersave);
                self.cpuctl.set_uclamp(Mode::Powersave)?;
                continue;
            }
            for (app, mode) in self.config.applist.clone() {
                if app_cache.clone().unwrap_or_default() != self.topapp.get()
                    && self.topapp.get() == app
                {
                    let mode = self.switch_mode(mode.as_str());
                    if self.config.app_launch_boost {
                        if !self.config.auto {
                            self.cpu.set_freq(Mode::Fast);
                        }
                        self.cpu.set_governor(Mode::Fast);
                        self.cpuctl.set_uclamp(Mode::Fast);
                        std::thread::sleep(std::time::Duration::from_secs(2));
                    }
                    log::info!("正在为{app}配置{mode}模式");
                    if !self.config.auto {
                        self.cpu.set_freq(mode);
                    }
                    self.cpu.set_governor(mode);
                    self.cpuctl.set_uclamp(mode)?;
                    app_cache = Some(app);
                } else {
                    let mode = self.switch_mode(self.config.osm.as_str());
                    if !self.config.auto {
                        self.cpu.set_freq(mode);
                    }
                    self.cpu.set_governor(mode);
                    self.cpuctl.set_uclamp(mode)?;
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
