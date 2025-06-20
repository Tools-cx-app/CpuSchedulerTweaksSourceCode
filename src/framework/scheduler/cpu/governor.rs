use std::path::Path;

use anyhow::Result;

use super::Cpu;
use crate::{
    framework::scheduler::Mode,
    utils::{files::write_with_locked, option_to_str},
};

pub trait CpuGovernor {
    fn set_governor(&self, mode: Mode);
    fn write_freq(&self, path: &Path, freq: String) -> Result<()>;
}

impl CpuGovernor for Cpu {
    /*
     * cpu控制器设置
     * 2025-06-07
     */
    fn set_governor(&self, mode: Mode) {
        let clusters = self.get_cluster_paths();
        let (has_big, has_middle, has_small, has_super_big) = clusters.check_existence();

        let get_governor = |cluster: &str| match mode {
            Mode::Powersave => match cluster {
                "big" => option_to_str(self.config.powersave.governor.big_cpu.clone()),
                "middle" => option_to_str(self.config.powersave.governor.middle_cpu.clone()),
                "small" => option_to_str(self.config.powersave.governor.small_cpu.clone()),
                "super_big" => option_to_str(self.config.powersave.governor.super_big_cpu.clone()),
                _ => String::new(),
            },
            Mode::Balance => match cluster {
                "big" => option_to_str(self.config.balance.governor.big_cpu.clone()),
                "middle" => option_to_str(self.config.balance.governor.middle_cpu.clone()),
                "small" => option_to_str(self.config.balance.governor.small_cpu.clone()),
                "super_big" => option_to_str(self.config.balance.governor.super_big_cpu.clone()),
                _ => String::new(),
            },
            Mode::Performance => match cluster {
                "big" => option_to_str(self.config.performance.governor.big_cpu.clone()),
                "middle" => option_to_str(self.config.performance.governor.middle_cpu.clone()),
                "small" => option_to_str(self.config.performance.governor.small_cpu.clone()),
                "super_big" => {
                    option_to_str(self.config.performance.governor.super_big_cpu.clone())
                }
                _ => String::new(),
            },
            Mode::Fast => match cluster {
                "big" => option_to_str(self.config.fast.governor.big_cpu.clone()),
                "middle" => option_to_str(self.config.fast.governor.middle_cpu.clone()),
                "small" => option_to_str(self.config.fast.governor.small_cpu.clone()),
                "super_big" => option_to_str(self.config.fast.governor.super_big_cpu.clone()),
                _ => String::new(),
            },
        };

        if has_big && let Some(path) = &clusters.big {
            let _ = self.write_freq(Path::new(path), get_governor("big"));
        }
        if has_middle && let Some(path) = &clusters.middle {
            let _ = self.write_freq(Path::new(path), get_governor("middle"));
        }
        if has_small && let Some(path) = &clusters.small {
            let _ = self.write_freq(Path::new(path), get_governor("small"));
        }
        if has_super_big && let Some(path) = &clusters.super_big {
            let _ = self.write_freq(Path::new(path), get_governor("super_big"));
        }
    }

    /*
     * 控制器写入
     * 2025-06-07
     */
    fn write_freq(&self, path: &Path, governor_content: String) -> Result<()> {
        let governor = path.join("scaling_governor");
        write_with_locked(&governor, &governor_content)?;
        Ok(())
    }
}
