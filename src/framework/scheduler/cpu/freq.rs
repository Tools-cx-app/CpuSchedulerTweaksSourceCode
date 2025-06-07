use std::{path::Path, sync::atomic::Ordering};

use anyhow::Result;

use super::Cpu;
use crate::{
    framework::scheduler::{DEBUG, Mode},
    utils::{files::write_with_locked, option_to_str},
};

pub trait CpuFreqs {
    fn set_freq(&self, mode: Mode);
    fn write_freq(&self, path: &Path, freq: Vec<u64>) -> Result<()>;
}

impl CpuFreqs for Cpu {
    /*
     * Cpu频率写入
     * 2025-06-07
     */
    fn set_freq(&self, mode: Mode) {
        let clusters = self.get_cluster_paths();
        let (has_big, has_middle, has_small, has_super_big) = clusters.check_existence();

        if DEBUG.load(Ordering::Relaxed) {
            clusters.debug_print();
        }

        let get_freqs = |cluster: &str| {
            let (min, max) = match mode {
                Mode::Powersave => match cluster {
                    "big" => (
                        option_to_str(self.config.powersave.freqs.big_cpu).min,
                        option_to_str(self.config.powersave.freqs.big_cpu).max,
                    ),
                    "middle" => (
                        option_to_str(self.config.powersave.freqs.middle_cpu).min,
                        option_to_str(self.config.powersave.freqs.middle_cpu).max,
                    ),
                    "small" => (
                        option_to_str(self.config.powersave.freqs.small_cpu).min,
                        option_to_str(self.config.powersave.freqs.small_cpu).max,
                    ),
                    "super_big" => (
                        option_to_str(self.config.powersave.freqs.super_big_cpu).min,
                        option_to_str(self.config.powersave.freqs.super_big_cpu).max,
                    ),
                    _ => (0, 0),
                },
                Mode::Balance => match cluster {
                    "big" => (
                        option_to_str(self.config.balance.freqs.big_cpu).min,
                        option_to_str(self.config.balance.freqs.big_cpu).max,
                    ),
                    "middle" => (
                        option_to_str(self.config.balance.freqs.middle_cpu).min,
                        option_to_str(self.config.balance.freqs.middle_cpu).max,
                    ),
                    "small" => (
                        option_to_str(self.config.balance.freqs.small_cpu).min,
                        option_to_str(self.config.balance.freqs.small_cpu).max,
                    ),
                    "super_big" => (
                        option_to_str(self.config.balance.freqs.super_big_cpu).min,
                        option_to_str(self.config.balance.freqs.super_big_cpu).max,
                    ),
                    _ => (0, 0),
                },
                Mode::Performance => match cluster {
                    "big" => (
                        option_to_str(self.config.performance.freqs.big_cpu).min,
                        option_to_str(self.config.performance.freqs.big_cpu).max,
                    ),
                    "middle" => (
                        option_to_str(self.config.performance.freqs.middle_cpu).min,
                        option_to_str(self.config.performance.freqs.middle_cpu).max,
                    ),
                    "small" => (
                        option_to_str(self.config.performance.freqs.small_cpu).min,
                        option_to_str(self.config.performance.freqs.small_cpu).max,
                    ),
                    "super_big" => (
                        option_to_str(self.config.performance.freqs.super_big_cpu).min,
                        option_to_str(self.config.performance.freqs.super_big_cpu).max,
                    ),
                    _ => (0, 0),
                },
                Mode::Fast => match cluster {
                    "big" => (
                        option_to_str(self.config.fast.freqs.big_cpu).min,
                        option_to_str(self.config.fast.freqs.big_cpu).max,
                    ),
                    "middle" => (
                        option_to_str(self.config.fast.freqs.middle_cpu).min,
                        option_to_str(self.config.fast.freqs.middle_cpu).max,
                    ),
                    "small" => (
                        option_to_str(self.config.fast.freqs.small_cpu).min,
                        option_to_str(self.config.fast.freqs.small_cpu).max,
                    ),
                    "super_big" => (
                        option_to_str(self.config.fast.freqs.super_big_cpu).min,
                        option_to_str(self.config.fast.freqs.super_big_cpu).max,
                    ),
                    _ => (0, 0),
                },
            };
            vec![max, min]
        };

        if has_big && let Some(path) = &clusters.big {
            let _ = self.write_freq(Path::new(path), get_freqs("big"));
        }
        if has_middle && let Some(path) = &clusters.middle {
            let _ = self.write_freq(Path::new(path), get_freqs("middle"));
        }
        if has_small && let Some(path) = &clusters.small {
            let _ = self.write_freq(Path::new(path), get_freqs("small"));
        }
        if has_super_big && let Some(path) = &clusters.super_big {
            let _ = self.write_freq(Path::new(path), get_freqs("super_big"));
        }
    }

    fn write_freq(&self, path: &Path, freq: Vec<u64>) -> Result<()> {
        if freq.len() < 2 {
            anyhow::bail!("无效的频率参数，需要最大和最小频率");
        }

        let max = path.join("scaling_max_freq");
        let min = path.join("scaling_min_freq");

        write_with_locked(&max, &freq[0].to_string())?;
        write_with_locked(&min, &freq[1].to_string())?;
        Ok(())
    }
}
