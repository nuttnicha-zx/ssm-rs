use std::env::consts::OS;
use std::fs;

/// Detect the Linux distribution name
pub fn get_os_name() -> String {
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
pub fn get_kernel() -> String {
    if let Ok(content) = fs::read_to_string("/proc/version") {
        if let Some(kernel_version) = content.split_whitespace().nth(2) {
            return kernel_version.to_string();
        }
    }
    "Unknown".to_string()
}
