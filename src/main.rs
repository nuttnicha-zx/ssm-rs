use std::thread::sleep;
use std::time::Duration;
use sysinfo::System;

fn main() {
    let mut sys = System::new_all();

    // Loop forever
    loop {
        sys.refresh_all();

        // Get system information
        let cpu_usage = sys.global_cpu_usage();
        let total_memory_bytes = sys.total_memory();
        let used_memory_bytes = sys.used_memory();
        let total_swap_bytes = sys.total_swap();
        let used_swap_bytes = sys.used_swap();

        // Calculate memory and swap usage in MB
        let total_memory = total_memory_bytes / 1024 / 1024;
        let used_memory = used_memory_bytes / 1024 / 1024;
        let total_swap = total_swap_bytes / 1024 / 1024;
        let used_swap = used_swap_bytes / 1024 / 1024;

        // Clear the terminal screen
        print!("\x1B[2J\x1B[1;1H");

        // Print system information
        println!("CPU usage: {:.0}%", cpu_usage);
        println!("Memory: {} MiB / {} MiB", used_memory, total_memory);
        println!("Swap: {} MiB / {} MiB", used_swap, total_swap);

        // Wait for 1 second before updating again
        sleep(Duration::from_millis(1000));
    }
}
