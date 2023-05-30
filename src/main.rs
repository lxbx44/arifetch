
use sysinfo::{SystemExt, CpuExt};
use whoami;
use std::time::Duration;
use std::env;
use std::path::Path;

fn main() {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();


    // host_name

    let host_name = sys.host_name().unwrap() + "@" + &whoami::username();

    //  DISTRO  

    let distro = sys.name().unwrap();

    // KERNEL

    let kernel = sys.kernel_version().unwrap();

    // UPTIME
    
    let duration = Duration::from_secs(sys.uptime());

    let days = duration.as_secs() / (60 * 60 * 24);
    let hours = (duration.as_secs() % (60 * 60 * 24)) / (60 * 60);
    let minutes = (duration.as_secs() % (60 * 60)) / 60;

    let uptime: String;

    if hours == 0 {
        uptime = minutes.to_string() + " minutes"
    } else if days == 0 {
        uptime = hours.to_string() + " hours, " + &minutes.to_string() + " minutes"
    } else {
        uptime = days.to_string() + " days, " + &hours.to_string() + " hours, " + &minutes.to_string() + " minutes"
    }

    // SHELL

    let shell_path = env::var("SHELL").unwrap();

    let shell = Path::new(&shell_path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("Unknown");

    // CPU
    
    let cpu_info = sys.cpus().iter().next().unwrap();
    let cpu = cpu_info.brand().to_string();

    // MEMORY 

    let used = sys.used_memory();
    let total = sys.total_memory();

    let used_mb = used / 1_048_576;
    let total_mb = total / 1_048_576;

    let memory = used_mb.to_string() + " MiB | " + &total_mb.to_string() + " MiB";


    println!("  
|------------------
|   {}
|------------------
|    OS: {}
|    Kernel: {}
|    Uptime: {}
|    Shell: {}
|    CPU: {}
|    Memory: {}
|------------------
             ", host_name, distro, kernel, uptime, shell, cpu, memory)
}
