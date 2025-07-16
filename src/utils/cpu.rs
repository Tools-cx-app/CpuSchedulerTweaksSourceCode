use std::{
    collections::HashMap,
    fs, io,
    time::{Duration, Instant},
};

pub struct CpuLoadUtils {
    last_cpu_state: HashMap<String, Vec<u64>>,
    last_core_loads: HashMap<i32, f64>,
    last_update_time: Option<Instant>,
}

impl CpuLoadUtils {
    pub fn new() -> io::Result<Self> {
        let last_cpu_state = Self::read_cpu_stats()?;
        Ok(Self {
            last_cpu_state,
            last_core_loads: HashMap::new(),
            last_update_time: None,
        })
    }

    fn read_cpu_stats() -> io::Result<HashMap<String, Vec<u64>>> {
        let content = fs::read_to_string("/proc/stat")?;
        let mut stats = HashMap::new();

        for line in content.lines() {
            if line.starts_with("cpu") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 5 {
                    continue;
                }

                let cpu_key = parts[0].to_string();
                let values: Vec<u64> = parts[1..].iter().filter_map(|s| s.parse().ok()).collect();

                stats.insert(cpu_key, values);
            }
        }

        Ok(stats)
    }

    fn total_time(stats: &[u64]) -> u64 {
        stats.iter().sum()
    }

    fn idle_time(stats: &[u64]) -> u64 {
        if stats.len() > 3 { stats[3] } else { 0 }
    }

    pub fn get_cpu_load(&mut self) -> io::Result<HashMap<i32, f64>> {
        if let Some(last_time) = self.last_update_time {
            if last_time.elapsed() < Duration::from_millis(500) {
                return Ok(self.last_core_loads.clone());
            }
        }

        let current_stats = Self::read_cpu_stats()?;
        let mut loads = HashMap::new();

        for (cpu_key, current_values) in &current_stats {
            if let Some(last_values) = self.last_cpu_state.get(cpu_key.as_str()) {
                if current_values.len() != last_values.len() {
                    continue;
                }

                let total_current = Self::total_time(&current_values);
                let idle_current = Self::idle_time(&current_values);

                let total_last = Self::total_time(last_values);
                let idle_last = Self::idle_time(last_values);

                let total_diff = total_current.checked_sub(total_last).unwrap_or(0);
                let idle_diff = idle_current.checked_sub(idle_last).unwrap_or(0);

                if total_diff == 0 {
                    loads.insert(self.cpu_key_to_id(&cpu_key), 0.0);
                    continue;
                }

                if idle_diff >= total_diff {
                    loads.insert(self.cpu_key_to_id(&cpu_key), 100.0);
                } else {
                    let load = 100.0 - (idle_diff as f64 * 100.0 / total_diff as f64);
                    loads.insert(self.cpu_key_to_id(&cpu_key), load);
                }
            }
        }

        self.last_cpu_state = current_stats;
        self.last_core_loads = loads.clone();
        self.last_update_time = Some(Instant::now());

        Ok(loads)
    }

    fn cpu_key_to_id(&self, key: &str) -> i32 {
        if key == "cpu" {
            -1
        } else {
            key[3..].parse().unwrap_or(-1)
        }
    }
}
