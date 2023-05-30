mod fetches;
use fetches::fetch;

fn main() {
    let fetch = fetch();
    let (days, hours, minutes) = fetch.uptime;

    let uptime = if hours == 0 {
        format!("{minutes} minutes")
    } else if days == 0 {
        format!("{hours} hours, {minutes} minutes")
    } else {
        format!("{days} days, {hours} hours, {minutes} minutes")
    };

    println!(
        "  
┌──────────────────
│  {}@{}
├──────────────────
│   OS: {}
│   Kernel: {}
│   Uptime: {}
│   Shell: {}
│   CPU: {}
│   Memory: {} MiB / {} MiB
└──────────────────
             ",
        fetch.username,
        fetch.hostname,
        fetch.os,
        fetch.kernel,
        uptime,
        fetch.shell,
        fetch.cpu,
        fetch.used_mem,
        fetch.total_mem
    );
}
