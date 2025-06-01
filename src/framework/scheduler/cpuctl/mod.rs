use std::path::Path;

use anyhow::Result;

use crate::{
    defs::TOP_APP_CPUCTL, framework::config::data::ConfigData, utils::files::write_with_locked,
};

use super::Mode;

pub struct CpuCtl {
    config: ConfigData,
}

impl CpuCtl {
    pub fn new() -> Self {
        Self {
            config: ConfigData::new(),
        }
    }

    /*
     * 配置文件加载
     * 2025-05-25
     */
    pub fn load_config(&mut self, config: ConfigData) {
        self.config = config;
    }

    /*
     * 设置uclamp
     * 2025-05-25
     */
    pub fn set_uclamp(&self, mode: Mode) -> Result<()> {
        let top_app_cpuctl = Path::new(TOP_APP_CPUCTL);
        let top_app_uclamp_max = top_app_cpuctl.join("cpu.uclamp.max");
        let top_app_uclamp_min = top_app_cpuctl.join("cpu.uclamp.min");
        let top_app_shares = top_app_cpuctl.join("cpu.shares");

        match mode {
            Mode::Powersave => {
                let _ = write_with_locked(
                    &top_app_uclamp_max,
                    self.config
                        .powersave
                        .cpuctl
                        .top_app
                        .uclamp
                        .max
                        .to_string()
                        .as_str(),
                );
                let _ = write_with_locked(
                    &top_app_uclamp_min,
                    self.config
                        .powersave
                        .cpuctl
                        .top_app
                        .uclamp
                        .min
                        .to_string()
                        .as_str(),
                );
                let _ = write_with_locked(
                    &top_app_shares,
                    self.config
                        .powersave
                        .cpuctl
                        .top_app
                        .shares
                        .to_string()
                        .as_str(),
                );
            }
            Mode::Balance => {
                let _ = write_with_locked(
                    &top_app_uclamp_max,
                    self.config
                        .balance
                        .cpuctl
                        .top_app
                        .uclamp
                        .max
                        .to_string()
                        .as_str(),
                );
                let _ = write_with_locked(
                    &top_app_uclamp_min,
                    self.config
                        .balance
                        .cpuctl
                        .top_app
                        .uclamp
                        .min
                        .to_string()
                        .as_str(),
                );
                let _ = write_with_locked(
                    &top_app_shares,
                    self.config
                        .balance
                        .cpuctl
                        .top_app
                        .shares
                        .to_string()
                        .as_str(),
                );
            }
            Mode::Performance => {
                let _ = write_with_locked(
                    &top_app_uclamp_max,
                    self.config
                        .performance
                        .cpuctl
                        .top_app
                        .uclamp
                        .max
                        .to_string()
                        .as_str(),
                );
                let _ = write_with_locked(
                    &top_app_uclamp_min,
                    self.config
                        .performance
                        .cpuctl
                        .top_app
                        .uclamp
                        .min
                        .to_string()
                        .as_str(),
                );
                let _ = write_with_locked(
                    &top_app_shares,
                    self.config
                        .performance
                        .cpuctl
                        .top_app
                        .shares
                        .to_string()
                        .as_str(),
                );
            }
            Mode::Fast => {
                let _ = write_with_locked(
                    &top_app_uclamp_max,
                    self.config
                        .fast
                        .cpuctl
                        .top_app
                        .uclamp
                        .max
                        .to_string()
                        .as_str(),
                );
                let _ = write_with_locked(
                    &top_app_uclamp_min,
                    self.config
                        .fast
                        .cpuctl
                        .top_app
                        .uclamp
                        .min
                        .to_string()
                        .as_str(),
                );
                let _ = write_with_locked(
                    &top_app_shares,
                    self.config.fast.cpuctl.top_app.shares.to_string().as_str(),
                );
            }
        }

        Ok(())
    }
}
