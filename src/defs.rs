use const_format::concatcp;
use libc::c_int;

pub const VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));
pub const SDC_SCHEDULER: &str = "/sys/class/block/sdc/queue/scheduler";
pub const SDC_READ_AHEAD: &str = "/sys/class/block/sdc/queue/read_ahead_kb";
pub const CPUSET: &str = "/dev/cpuset/";
pub const CONFIG_PATH: &str = "/data/adb/modules/CpuSchedulerTweaks/config.toml";
pub const MOD_PROP_PATH: &str = "/data/adb/modules/CpuSchedulerTweaks/module.prop";
pub const BOOST_PATHS: [&str; 43] = [
    "/sys/power/pnpmgr/touch_boost",
    "/sys/power/pnpmgr/long_duration_touch_boost",
    "/sys/kernel/ems/eff_mode",
    "/sys/kernel/hmp/boost",
    "/sys/kernel/hmp/boostpulse_duration",
    "/sys/kernel/intelli_plug/intelli_plug_active",
    "/sys/kernel/zen_decision/enabled",
    "/sys/devices/system/cpu/sched/sched_boost",
    "/sys/devices/system/cpu/cpuhotplug/enabled",
    "/sys/devices/system/cpu/hyp_core_ctl/enable",
    "/sys/devices/virtual/misc/mako_hotplug_control/enabled",
    "/sys/module/msm_performance/parameters/touchboost",
    "/sys/module/msm_thermal/vdd_restriction/enabled",
    "/sys/module/msm_thermal/core_control/enabled",
    "/sys/module/aigov/parameters/enable",
    "/sys/module/opchain/parameters/chain_on",
    "/sys/module/blu_plug/parameters/enabled",
    "/sys/module/autosmp/parameters/enabled",
    "/proc/mz_thermal_boost/sched_boost_enabled",
    "/proc/mz_scheduler/vip_task/enabled",
    "/proc/sys/fbg/frame_boost_enabled",
    "/proc/sys/fbg/slide_boost_enabled",
    "/sys/module/fbt_cpu/parameters/boost_affinity*",
    "/sys/module/mtk_fpsgo/parameters/boost_affinity*",
    "/sys/module/mtk_fpsgo/parameters/perfmgr_enable",
    "/sys/module/perfmgr/parameters/perfmgr_enable",
    "/sys/module/perfmgr_policy/parameters/perfmgr_enable",
    "/sys/kernel/fpsgo/common/fpsgo_enable",
    "/sys/kernel/debug/fpsgo/common/force_onoff",
    "/sys/kernel/ged/hal/dcs_mode",
    "/proc/perfmgr/tchbst/user/usrtch",
    "/sys/kernel/fpsgo/fbt/thrm_temp_th",
    "/sys/kernel/fpsgo/fbt/thrm_limit_cpu",
    "/sys/kernel/fpsgo/fbt/thrm_sub_cpu",
    "/proc/perfmgr/syslimiter/syslimiter_force_disable",
    "/sys/module/mtk_core_ctl/parameters/policy_enable",
    "/sys/kernel/fpsgo/fbt/switch_idleprefer",
    "/sys/module/devfreq_boost/parameters/*",
    "/sys/devices/system/cpu/cpu*/sched_load_boost",
    "/sys/devices/system/cpu/cpu_boost/*",
    "/sys/devices/system/cpu/cpu_boost/parameters/*",
    "/sys/module/cpu_boost/parameters/*",
    "/sys/module/dsboost/parameters/*",
];
pub const RESET_TIME: std::time::Duration = std::time::Duration::from_millis(500);
pub const TOP_APP_CPUCTL: &str = "/dev/cpuctl/top-app/";
pub const GET_KERNELSU_VERSION: c_int = 2;
pub const KERNEL_SU_OPTION: u32 = 0xDEADBEEF;
