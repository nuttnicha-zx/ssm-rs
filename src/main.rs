mod os;
mod system;
mod ui;

use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use sysinfo::System;

use ui::TextColor as C;

fn main() {
    let mut system_info = System::new_all();

    // Get static information
    let os_name = os::get_os_name();
    let kernel = os::get_kernel();

    ui::hide_cursor();

    ctrlc::set_handler(move || {
        ui::show_cursor();
        exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    // Main loop to refresh and display system information
    loop {
        // Refresh system information
        system_info.refresh_all();

        let (cpu_usage, cpu_clocks) = system::get_cpu_usage(&system_info);
        let (used_memory, total_memory, memory_usage) = system::get_memory_usage(&system_info);
        let (used_swap, total_swap, swap_usage) = system::get_swap_usage(&system_info);

        // Print system information
        ui::clear_screen();
        println!("    {}OS{}:{} {}", C::Blue, C::Gray, C::Reset, os_name);
        println!("{}Kernel{}:{} {}", C::Blue, C::Gray, C::Reset, kernel);
        println!(
            "   {}CPU{}:{} {} {}({}{}{}){}",
            C::Blue,
            C::Gray,
            C::Reset,
            cpu_usage,
            C::Gray,
            C::Reset,
            cpu_clocks,
            C::Gray,
            C::Reset,
        );
        println!(
            "{}Memory{}:{} {}{}/{}{} GB {}({}{}{}){}",
            C::Blue,
            C::Gray,
            C::Reset,
            used_memory,
            C::Gray,
            C::Reset,
            total_memory,
            C::Gray,
            C::Reset,
            memory_usage,
            C::Gray,
            C::Reset,
        );
        println!(
            "  {}Swap{}:{} {}{}/{}{} GB {}({}{}{}){}",
            C::Blue,
            C::Gray,
            C::Reset,
            used_swap,
            C::Gray,
            C::Reset,
            total_swap,
            C::Gray,
            C::Reset,
            swap_usage,
            C::Gray,
            C::Reset,
        );

        // Wait for 1 second before updating again
        sleep(Duration::from_secs(1));
    }
}
