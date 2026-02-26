mod os;
mod system;
mod ui;

use std::thread::sleep;
use std::time::Duration;
use sysinfo::System;

use crate::ui::TextColor;

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
        println!(
            "    {}OS{}:{} {}",
            TextColor::Blue,
            TextColor::White,
            TextColor::Reset,
            os_name
        );
        println!(
            "{}Kernel{}:{} {}",
            TextColor::Blue,
            TextColor::White,
            TextColor::Reset,
            kernel
        );
        println!(
            "   {}CPU{}:{} {} {}({}{}{}){}",
            TextColor::Blue,
            TextColor::White,
            TextColor::Reset,
            cpu_usage,
            TextColor::White,
            TextColor::Reset,
            cpu_clocks,
            TextColor::White,
            TextColor::Reset,
        );
        println!(
            "{}Memory{}:{} {}{}/{}{} MB {}({}{}{}){}",
            TextColor::Blue,
            TextColor::White,
            TextColor::Reset,
            used_memory,
            TextColor::White,
            TextColor::Reset,
            total_memory,
            TextColor::White,
            TextColor::Reset,
            memory_usage,
            TextColor::White,
            TextColor::Reset,
        );
        println!(
            "  {}Swap{}:{} {}{}/{}{} MB {}({}{}{}){}",
            TextColor::Blue,
            TextColor::White,
            TextColor::Reset,
            used_swap,
            TextColor::White,
            TextColor::Reset,
            total_swap,
            TextColor::White,
            TextColor::Reset,
            swap_usage,
            TextColor::White,
            TextColor::Reset,
        );

        // Wait for 1 second before updating again
        sleep(Duration::from_millis(1000));
    }
}
