mod os;
mod system;
mod ui;

use std::thread::sleep;
use std::time::Duration;
use sysinfo::System;

fn main() {
    let mut system_info = System::new_all();

    // Get static information
    let os_name = os::get_os_name();
    let kernel = os::get_kernel();

    // Main loop to refresh and display system information
    loop {
        // Refresh system information
        system_info.refresh_all();

        let (cpu_usage, cpu_clocks) = system::get_cpu_usage(&system_info);
        let (used_memory, total_memory, memory_usage) = system::get_memory_usage(&system_info);
        let (used_swap, total_swap, swap_usage) = system::get_swap_usage(&system_info);

        // Print system information
        ui::clear_screen();
        println!("    OS: {}", os_name);
        println!("Kernel: {}", kernel);
        println!("   CPU: {} ({})", cpu_usage, cpu_clocks);
        println!(
            "Memory: {}/{} MB ({})",
            used_memory, total_memory, memory_usage
        );
        println!("  Swap: {}/{} MB ({})", used_swap, total_swap, swap_usage);

        // Wait for 1 second before updating again
        sleep(Duration::from_millis(1000));
    }
}
