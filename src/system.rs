use sysinfo::System;

/// Get CPU usage information
pub fn get_cpu_usage(sys: &System) -> (String, String) {
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
pub fn get_memory_usage(sys: &System) -> (String, String, String) {
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
pub fn get_swap_usage(sys: &System) -> (String, String, String) {
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
