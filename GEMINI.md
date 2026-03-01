# GEMINI.md - ssm-rs Project Context

## Project Overview
`ssm-rs` (Simple System Monitor) is a lightweight, real-time system monitoring tool developed in Rust. It provides a clean, minimal terminal interface to view essential system resource usage at a glance.

- **Purpose:** Provide a simple, uncluttered live view of system metrics.
- **Main Technologies:**
    - **Language:** Rust (2024 Edition)
    - **Libraries:**
        - `sysinfo`: Core library for cross-platform system information gathering.
        - `ctrlc`: For graceful signal handling (Ctrl+C).
- **Architecture:**
    - `src/main.rs`: Orchestrates the main loop, refreshes data, and handles UI updates.
    - `src/os.rs`: Linux-specific logic for detecting OS distribution and kernel version.
    - `src/system.rs`: Wraps `sysinfo` to provide formatted CPU, Memory, and Swap usage strings.
    - `src/ui.rs`: Manages terminal state using ANSI escape sequences (colors, cursor visibility, screen clearing).

## Building and Running
The project follows standard Rust/Cargo conventions.

- **Run in Development:**
  ```bash
  cargo run
  ```
- **Build Release Binary:**
  ```bash
  cargo build --release
  ```
- **Run Tests:**
  ```bash
  cargo test
  ```
- **Linting:**
  ```bash
  cargo clippy
  ```

## Development Conventions
- **Minimalism:** Keep the UI uncluttered. Avoid adding heavy dependencies for the TUI; prefer ANSI escape sequences where possible.
- **Modularity:** Keep system data gathering (`system.rs`), OS detection (`os.rs`), and UI logic (`ui.rs`) separate from the main execution loop.
- **Error Handling:** Use `expect` or proper `Result` handling for critical path setup (like the Ctrl-C handler).
- **Performance:** Ensure the monitor itself has a minimal resource footprint.

## Roadmap & Future Direction
- **Configuration:** Planned support for `ssm.toml` to toggle modules and customize colors.
- **Metrics:** Optional compact indicators for Network, Disk, Temperature, and Battery.
- **UI Polish:** Reducing screen flicker and adding an optional "Top Processes" view.
