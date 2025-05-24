#![allow(unused_assignments)]

use std::{
    fs::{Permissions, set_permissions, write},
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

use crate::framework::scheduler::Mode;

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
        let big_path = PathBuf::from(format!(
            "/sys/devices/system/cpu/cpufreq/policy{}",
            self.config.cpu_config.big
        ));
        let big = big_path.as_path();

        let middle_path = PathBuf::from(format!(
            "/sys/devices/system/cpu/cpufreq/policy{}",
            self.config.cpu_config.middle
        ));
        let middle = middle_path.as_path();

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

        if !big.exists() {
            log::error!("CPU簇{}不存在", self.config.cpu_config.big);
            return;
        }
        if !middle.exists() {
            log::error!("CPU簇{}不存在", self.config.cpu_config.middle);
            return;
        }
        let has_small_big = small.map(|p| p.exists()).unwrap_or(false);
        let has_super_big = super_big.map(|p| p.exists()).unwrap_or(false);

        #[cfg(debug_assertions)]
        {
            log::debug!("big簇: {}", big.display());
            log::debug!("middle簇: {}", middle.display());
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
                big_governor = self.config.powersave.big_cpu_governor.clone();
                middle_governor = self.config.powersave.middle_cpu_governor.clone();
                if has_small_big {
                    small_governor =
                        self.option_to_string(self.config.powersave.small_cpu_governor.clone());
                }
                if has_super_big {
                    super_big_governor =
                        self.option_to_string(self.config.powersave.super_big_cpu_governor.clone())
                }
            }
            Mode::Balance => {
                big_governor = self.config.balance.big_cpu_governor.clone();
                middle_governor = self.config.balance.middle_cpu_governor.clone();
                if has_small_big {
                    small_governor =
                        self.option_to_string(self.config.balance.small_cpu_governor.clone());
                }
                if has_super_big {
                    super_big_governor =
                        self.option_to_string(self.config.balance.super_big_cpu_governor.clone())
                }
            }
            Mode::Performance => {
                big_governor = self.config.performance.big_cpu_governor.clone();
                middle_governor = self.config.performance.middle_cpu_governor.clone();
                if has_small_big {
                    small_governor =
                        self.option_to_string(self.config.performance.small_cpu_governor.clone());
                }
                if has_super_big {
                    super_big_governor = self
                        .option_to_string(self.config.performance.super_big_cpu_governor.clone())
                }
            }
            Mode::Fast => {
                big_governor = self.config.fast.big_cpu_governor.clone();
                middle_governor = self.config.fast.middle_cpu_governor.clone();
                if has_small_big {
                    small_governor =
                        self.option_to_string(self.config.fast.small_cpu_governor.clone());
                }
                if has_super_big {
                    super_big_governor =
                        self.option_to_string(self.config.fast.super_big_cpu_governor.clone())
                }
            }
        }
        let _ = self.write_freq(big, big_governor);
        let _ = self.write_freq(middle, middle_governor);
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

        set_permissions(&gonvernor, Permissions::from_mode(0o644))
            .context("无法设置最大频率权限")?;
        write(&gonvernor, gonvernor_content).context("无法写入最大频率")?;
        set_permissions(&gonvernor, Permissions::from_mode(0o400))
            .context("无法恢复最大频率权限")?;
        Ok(())
    }
}
