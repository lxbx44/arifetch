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
    ╭───────────────┄ 
    │  \u{eb99} \u{ea9f} {}@{}
    ├───────────────┄ 
    │   OS      \u{e712}   {}
    │   Kernel  \u{f109}   {}
    │   Uptime  \u{e385}   {}
    │   Shell   \u{f489}   {}
    │   CPU     \u{f4bc}   {}
    │   Memory  \u{eace}   {} MiB / {} MiB
    ╰───────────────┄ 
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
