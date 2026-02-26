# ssm-rs

A simple, real-time system monitoring tool written in Rust.

`ssm-rs` (Simple System Monitor) displays essential system information directly in your terminal, updating every second to give you a live view of your system's resource usage.

## Features

- **Live Updates**: Refreshes system stats every second.
- **Resource Tracking**: Displays current usage for:
  - OS Name
  - Kernel Version
  - CPU Usage & Clock Speed
  - Memory (RAM) Usage
  - Swap Usage
- **Clean UI**: Uses ANSI escape sequences for a clean, colored display without a persistent cursor.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024 or later)

## Installation & Running

Clone the repository and run the project using `cargo`:

```bash
cargo run
```

To build a release version for better performance:

```bash
cargo build --release
./target/release/ssm-rs
```

## Rust Dependencies

- `sysinfo`: For gathering cross-platform system information.
- `ctrlc`: For safe signal handling (Ctrl+C).
