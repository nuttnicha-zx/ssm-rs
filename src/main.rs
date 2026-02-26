use std::env::consts::OS;
use std::fs;
use std::thread::sleep;
use std::time::Duration;
use sysinfo::System;

/// Clear terminal screen
fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

/// Detect the Linux distribution name
fn get_os_name() -> String {
    // Try to read /etc/os-release first (preferred method)
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if line.starts_with("PRETTY_NAME=") {
                // Extract the value between quotes
                if let Some(value) = line.split('=').nth(1) {
                    return value.trim_matches('"').to_string();
                }
            }
        }
    }

    // Fallback to /etc/lsb-release
    if let Ok(content) = fs::read_to_string("/etc/lsb-release") {
        for line in content.lines() {
            if line.starts_with("DISTRIB_DESCRIPTION=") {
                if let Some(value) = line.split('=').nth(1) {
                    return value.trim_matches('"').to_string();
                }
            }
        }
    }

    // Default fallback
    OS.to_string()
}

/// Get the kernel version
fn get_kernel() -> String {
    if let Ok(content) = fs::read_to_string("/proc/version") {
        if let Some(kernel_version) = content.split_whitespace().nth(2) {
            return kernel_version.to_string();
        }
    }
    "Unknown".to_string()
}

/// Get CPU usage information
fn get_cpu_usage(sys: &System) -> String {
    let cpu_usage = sys.global_cpu_usage();
    format!("CPU Usage: {:.0}%", cpu_usage)
}

/// Get memory usage information
fn get_memory_usage(sys: &System) -> String {
    let total_memory_bytes = sys.total_memory();
    let used_memory_bytes = sys.used_memory();

    // Calculate memory usage in MB
    let total_memory = total_memory_bytes / 1024 / 1024;
    let used_memory = used_memory_bytes / 1024 / 1024;

    format!("Memory Usage: {}/{} MB", used_memory, total_memory)
}

/// Get swap usage information
fn get_swap_usage(sys: &System) -> String {
    let total_swap_bytes = sys.total_swap();
    let used_swap_bytes = sys.used_swap();

    // Calculate swap usage in MB
    let total_swap = total_swap_bytes / 1024 / 1024;
    let used_swap = used_swap_bytes / 1024 / 1024;

    format!("Swap Usage: {}/{} MB", used_swap, total_swap)
}

fn main() {
    let mut system = System::new_all();

    // Get static information
    let os_name = get_os_name();
    let kernel = get_kernel();

    // Main loop to refresh and display system information
    loop {
        // Refresh system information
        system.refresh_all();

        // Print system information
        clear_screen();
        println!("OS: {}", os_name);
        println!("Kernel: {}", kernel);
        println!("CPU usage: {}", get_cpu_usage(&system));
        println!("Memory: {}", get_memory_usage(&system));
        println!("Swap: {}", get_swap_usage(&system));

        // Wait for 1 second before updating again
        sleep(Duration::from_millis(1000));
    }
}
