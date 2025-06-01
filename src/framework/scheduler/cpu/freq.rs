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

pub trait CpuFreqs {
    fn set_freq(&self, mode: Mode);
    fn write_freq(&self, path: &Path, freq: Vec<u64>) -> Result<()>;
}

impl CpuFreqs for Cpu {
    /*
     * cpu频率设置
     * 2025-05-24
     */
    fn set_freq(&self, mode: Mode) {
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

        let mut big_freq = Vec::new();
        let mut middle_freq = Vec::new();
        let mut small_freq = Vec::new();
        let mut super_big_freq = Vec::new();

        /*
         * 自动判断，并按顺序写入频率变量
         * 2025-05-24
         */
        match mode {
            Mode::Powersave => {
                if has_big {
                    big_freq.extend(&[
                        option_to_str(self.config.powersave.freqs.big_cpu).max,
                        option_to_str(self.config.powersave.freqs.big_cpu).min,
                    ]);
                }
                if has_middle {
                    middle_freq.extend(&[
                        option_to_str(self.config.powersave.freqs.middle_cpu).max,
                        option_to_str(self.config.powersave.freqs.middle_cpu).min,
                    ]);
                }
                if has_small_big {
                    small_freq.extend(&[
                        option_to_str(self.config.powersave.freqs.small_cpu).max,
                        option_to_str(self.config.powersave.freqs.small_cpu).min,
                    ]);
                }
                if has_super_big {
                    super_big_freq.extend(&[
                        option_to_str(self.config.powersave.freqs.super_big_cpu).max,
                        option_to_str(self.config.powersave.freqs.super_big_cpu).min,
                    ]);
                }
            }
            Mode::Balance => {
                if has_big {
                    big_freq.extend(&[
                        option_to_str(self.config.balance.freqs.big_cpu).max,
                        option_to_str(self.config.balance.freqs.big_cpu).min,
                    ]);
                }
                if has_middle {
                    middle_freq.extend(&[
                        option_to_str(self.config.balance.freqs.middle_cpu).max,
                        option_to_str(self.config.balance.freqs.middle_cpu).min,
                    ]);
                }
                if has_small_big {
                    small_freq.extend(&[
                        option_to_str(self.config.balance.freqs.small_cpu).max,
                        option_to_str(self.config.balance.freqs.small_cpu).min,
                    ]);
                }
                if has_super_big {
                    super_big_freq.extend(&[
                        option_to_str(self.config.balance.freqs.super_big_cpu).max,
                        option_to_str(self.config.balance.freqs.super_big_cpu).min,
                    ]);
                }
            }
            Mode::Performance => {
                if has_big {
                    big_freq.extend(&[
                        option_to_str(self.config.performance.freqs.big_cpu).max,
                        option_to_str(self.config.performance.freqs.big_cpu).min,
                    ]);
                }
                if has_middle {
                    middle_freq.extend(&[
                        option_to_str(self.config.performance.freqs.middle_cpu).max,
                        option_to_str(self.config.performance.freqs.middle_cpu).min,
                    ]);
                }
                if has_small_big {
                    small_freq.extend(&[
                        option_to_str(self.config.performance.freqs.small_cpu).max,
                        option_to_str(self.config.performance.freqs.small_cpu).min,
                    ]);
                }
                if has_super_big {
                    super_big_freq.extend(&[
                        option_to_str(self.config.performance.freqs.super_big_cpu).max,
                        option_to_str(self.config.performance.freqs.super_big_cpu).min,
                    ]);
                }
            }
            Mode::Fast => {
                if has_big {
                    big_freq.extend(&[
                        option_to_str(self.config.fast.freqs.big_cpu).max,
                        option_to_str(self.config.fast.freqs.big_cpu).min,
                    ]);
                }
                if has_middle {
                    middle_freq.extend(&[
                        option_to_str(self.config.fast.freqs.middle_cpu).max,
                        option_to_str(self.config.fast.freqs.middle_cpu).min,
                    ]);
                }
                if has_small_big {
                    small_freq.extend(&[
                        option_to_str(self.config.fast.freqs.small_cpu).max,
                        option_to_str(self.config.fast.freqs.small_cpu).min,
                    ]);
                }
                if has_super_big {
                    super_big_freq.extend(&[
                        option_to_str(self.config.fast.freqs.super_big_cpu).max,
                        option_to_str(self.config.fast.freqs.super_big_cpu).min,
                    ]);
                }
            }
        }

        if has_big {
            if let Some(b) = big {
                let _ = self.write_freq(b, big_freq);
            }
        }
        if has_middle {
            if let Some(m) = middle {
                let _ = self.write_freq(m, middle_freq);
            }
        }
        if has_small_big {
            if let Some(s) = small {
                let _ = self.write_freq(s, small_freq);
            }
        }
        if has_super_big {
            if let Some(sb) = super_big {
                let _ = self.write_freq(sb, super_big_freq);
            }
        }
    }

    /*
     * 频率写入部分
     * 2025-05-24
     */
    fn write_freq(&self, path: &Path, freq: Vec<u64>) -> Result<()> {
        let max = path.join("/scaling_max_freq");
        let min = path.join("/scaling_min_freq");

        if freq.len() < 2 {
            anyhow::bail!("无效的频率参数，需要最大和最小频率");
        }

        let _ = write_with_locked(max, freq[0].to_string().as_str());
        let _ = write_with_locked(min, freq[1].to_string().as_str());
        Ok(())
    }
}
