use std::{env, time::Duration, process::Command};
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
    
    // Gets the shell path from the enviroment variables
    let shell_path = env::var("SHELL");
    
    let shell = match shell_path {
        // If the envoriment variable exists, give the last item on the path which should be the binary
        Ok(shell) => shell.split('/').last().unwrap_or(&def).to_string(),
        Err(_) => {
            // If the envoriment variable doesn't exist and you're on windows, check if you're
            // using Powershell or CMD
            if cfg!(target_family = "windows") {
                let output = match Command::new("echo").arg("%PATH%").output() {
                    Ok(output) => {
                        println!("There is output!");
                        String::from_utf8(output.stdout).unwrap_or(def.clone())
                    }
                    Err(err) => {
                        println!("There is no output, {err}");
                        def.clone()
                    }
                };

                // "Echo %PATH%" on Powershell will just "%PATH%". On CMD it outputs the PATH
                // envoriment variable.
                if output == *"%PATH%" {
                    String::from("Powershell")
                } else {
                    String::from("CMD")
                }

            } else {
                def.clone()
            }
        }
    };

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
        total_mem: sys.total_memory() / 1_048_576,
    }
}
