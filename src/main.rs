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
fn get_cpu_usage(sys: &System) -> (String, String) {
    let cpu_usage = sys.global_cpu_usage();
    let cpu_clocks_mhz =
        sys.cpus().iter().map(|cpu| cpu.frequency()).sum::<u64>() / sys.cpus().len() as u64;
    let cpu_clocks_ghz = cpu_clocks_mhz as f64 / 1000.0;

    (
        format!("{:.1}%", cpu_usage),
        format!("{:.1} GHz", cpu_clocks_ghz),
    )
}

/// Get memory usage information
fn get_memory_usage(sys: &System) -> (String, String, String) {
    let total_memory_bytes = sys.total_memory();
    let used_memory_bytes = sys.used_memory();

    // Calculate memory usage in MB
    let total_memory = total_memory_bytes / 1024 / 1024;
    let used_memory = used_memory_bytes / 1024 / 1024;
    let memory_usage_percentage = (used_memory as f64 / total_memory as f64) * 100.0;

    (
        format!("{}", used_memory),
        format!("{}", total_memory),
        format!("{:.1}%", memory_usage_percentage),
    )
}

/// Get swap usage information
fn get_swap_usage(sys: &System) -> (String, String, String) {
    let total_swap_bytes = sys.total_swap();
    let used_swap_bytes = sys.used_swap();

    // Calculate swap usage in MB
    let total_swap = total_swap_bytes / 1024 / 1024;
    let used_swap = used_swap_bytes / 1024 / 1024;
    let swap_usage_percentage = if total_swap > 0 {
        (used_swap as f64 / total_swap as f64) * 100.0
    } else {
        0.0
    };

    (
        format!("{}", used_swap),
        format!("{}", total_swap),
        format!("{:.1}%", swap_usage_percentage),
    )
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

        let (cpu_usage, cpu_clocks) = get_cpu_usage(&system);
        let (used_memory, total_memory, memory_usage) = get_memory_usage(&system);
        let (used_swap, total_swap, swap_usage) = get_swap_usage(&system);

        // Print system information
        clear_screen();
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
