use std::{env, time::Duration};
use sysinfo::{CpuExt, System, SystemExt};
use whoami::username;

pub struct Fetch {
    pub hostname: String,
    pub username: String,
    pub os: String,
    pub kernel: String,
    pub uptime: (u64, u64, u64),
    pub shell: String,
    pub cpu: String,
    pub used_mem: u64,
    pub total_mem: u64,
}

pub fn fetch() -> Fetch {
    let mut sys = System::new_all();
    let def = String::from("Unkown");
    sys.refresh_all();

    let shell_path = env::var("SHELL").unwrap_or_else(|_| def.clone());
    let shell = shell_path
        .split('/')
        .last()
        .unwrap_or("Unknown")
        .to_string();

    let cpu_info = sys.cpus().iter().next();
    let cpu = cpu_info.map_or_else(|| def.clone(), |info| info.brand().to_string());

    let duration = Duration::from_secs(sys.uptime()).as_secs();

    let days = duration / (60 * 60 * 24);
    let hours = duration % (60 * 60 * 24) / (60 * 60);
    let minutes = duration % (60 * 60) / 60;

    Fetch {
        hostname: sys.host_name().unwrap_or_else(|| def.clone()),
        username: username(),
        os: sys.name().unwrap_or_else(|| def.clone()),
        kernel: sys.kernel_version().unwrap_or(def),
        uptime: (days, hours, minutes),
        shell,
        cpu,
        used_mem: sys.used_memory() / 1_048_576,
        total_mem: sys.free_memory() / 1_048_576,
    }
}
