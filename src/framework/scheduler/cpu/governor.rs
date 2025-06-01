#![allow(unused_assignments)]

use std::{
    path::{Path, PathBuf},
    sync::atomic::Ordering,
};

use anyhow::Result;

use crate::{
    framework::scheduler::{DEBUG, Mode},
    utils::{files::write_with_locked, option_to_str},
};

use super::Cpu;

pub trait CpuGovernor {
    fn set_governor(&self, mode: Mode);
    fn write_freq(&self, path: &Path, freq: String) -> Result<()>;
}

impl CpuGovernor for Cpu {
    /*
     * cpu控制器设置
     * 2025-05-24
     */
    fn set_governor(&self, mode: Mode) {
        let big_path = self
            .config
            .cpu_config
            .big
            .map(|id| PathBuf::from(format!("/sys/devices/system/cpu/cpufreq/policy{id}")));
        let big = big_path.as_ref().map(|p| p.as_path());

        let middle_path = self
            .config
            .cpu_config
            .middle
            .map(|id| PathBuf::from(format!("/sys/devices/system/cpu/cpufreq/policy{id}")));
        let middle = middle_path.as_ref().map(|p| p.as_path());

        let small_path = self
            .config
            .cpu_config
            .small
            .map(|id| PathBuf::from(format!("/sys/devices/system/cpu/cpufreq/policy{id}")));
        let small = small_path.as_ref().map(|p| p.as_path());

        let super_big_path = self
            .config
            .cpu_config
            .super_big
            .map(|id| PathBuf::from(format!("/sys/devices/system/cpu/cpufreq/policy{id}")));
        let super_big = super_big_path.as_ref().map(|p| p.as_path());

        let has_big = big.map(|p| p.exists()).unwrap_or(false);
        let has_middle = middle.map(|p| p.exists()).unwrap_or(false);
        let has_small_big = small.map(|p| p.exists()).unwrap_or(false);
        let has_super_big = super_big.map(|p| p.exists()).unwrap_or(false);

        if DEBUG.load(Ordering::Relaxed) {
            if let Some(b) = big {
                log::debug!("big簇: {}", b.display());
            }
            if let Some(m) = middle {
                log::debug!("middle簇: {}", m.display());
            }
            if let Some(s) = small {
                log::debug!("small簇: {}", s.display());
            }
            if let Some(sb) = super_big {
                log::debug!("super_big簇: {}", sb.display());
            }
        }

        let mut small_governor = String::new();
        let mut middle_governor = String::new();
        let mut big_governor = String::new();
        let mut super_big_governor = String::new();
        /*
         * 自动判断
         */
        match mode {
            Mode::Powersave => {
                if has_big {
                    big_governor = option_to_str(self.config.powersave.governor.big_cpu.clone());
                }
                if has_middle {
                    middle_governor =
                        option_to_str(self.config.powersave.governor.middle_cpu.clone());
                }
                if has_small_big {
                    small_governor =
                        option_to_str(self.config.powersave.governor.small_cpu.clone());
                }
                if has_super_big {
                    super_big_governor =
                        option_to_str(self.config.powersave.governor.super_big_cpu.clone())
                }
            }
            Mode::Balance => {
                if has_big {
                    big_governor = option_to_str(self.config.balance.governor.big_cpu.clone());
                }
                if has_middle {
                    middle_governor =
                        option_to_str(self.config.balance.governor.middle_cpu.clone());
                }
                if has_small_big {
                    small_governor = option_to_str(self.config.balance.governor.small_cpu.clone());
                }
                if has_super_big {
                    super_big_governor =
                        option_to_str(self.config.balance.governor.super_big_cpu.clone())
                }
            }
            Mode::Performance => {
                if has_big {
                    big_governor = option_to_str(self.config.performance.governor.big_cpu.clone());
                }
                if has_middle {
                    middle_governor =
                        option_to_str(self.config.performance.governor.middle_cpu.clone());
                }
                if has_small_big {
                    small_governor =
                        option_to_str(self.config.performance.governor.small_cpu.clone());
                }
                if has_super_big {
                    super_big_governor =
                        option_to_str(self.config.performance.governor.super_big_cpu.clone())
                }
            }
            Mode::Fast => {
                if has_big {
                    big_governor = option_to_str(self.config.fast.governor.big_cpu.clone());
                }
                if has_middle {
                    middle_governor = option_to_str(self.config.fast.governor.middle_cpu.clone());
                }
                if has_small_big {
                    small_governor = option_to_str(self.config.fast.governor.small_cpu.clone());
                }
                if has_super_big {
                    super_big_governor =
                        option_to_str(self.config.fast.governor.super_big_cpu.clone())
                }
            }
        }
        if has_big {
            if let Some(b) = big {
                let _ = self.write_freq(b, big_governor);
            }
        }
        if has_middle {
            if let Some(m) = middle {
                let _ = self.write_freq(m, middle_governor);
            }
        }
        if has_small_big {
            if let Some(s) = small {
                let _ = self.write_freq(s, small_governor);
            }
        }
        if has_super_big {
            if let Some(sb) = super_big {
                let _ = self.write_freq(sb, super_big_governor);
            }
        }
    }

    /*
     * 控制器写入
     * 2025-05-24
     */
    fn write_freq(&self, path: &Path, gonvernor_content: String) -> Result<()> {
        let gonvernor = path.join("scaling_governor");

        let _ = write_with_locked(&gonvernor, gonvernor_content.as_str());
        Ok(())
    }
}
