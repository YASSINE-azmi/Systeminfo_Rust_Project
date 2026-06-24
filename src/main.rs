use sysinfo::{System, Disks};
fn main() {
    let mut sys = System::new_all();

    sys.refresh_all();

    if let Some(os_name) = System::name(){
        println!("Your system is : {}", os_name);
    } else {
        println!("System is not available");
    }

    println!("total memory: {} bytes", sys.total_memory());
    println!("used memory : {} bytes", sys.used_memory());
    println!("total swap  : {} bytes", sys.total_swap());
    println!("used swap   : {} bytes", sys.used_swap());

    println!("NB CPUs: {}", sys.cpus().len());

    println!("=> disks:");
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        println!("{disk:?}");
    }

    println!("\n==========================================================================");
    println!("| {:<8} | {:<30} | {:<30} |", "PID", "Process Name", "Disk Usage");
    println!("==========================================================================");

    for (pid, process) in sys.processes() {
        let disk_usage_str = format!("{:?}", process.disk_usage());
        println!("| {:<8} | {:<30} | {:<30} |", pid.to_string(), process.name().to_string_lossy(), disk_usage_str);
    }
}