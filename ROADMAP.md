# Project's Roadmap
This document outlines the roadmap for the development of the `ssm-rs` project. It includes planned features, milestones, and timelines for the upcoming releases.

**Core Philosophy:** `ssm-rs` aims to be a Minimal System Monitor where you can get almost all important system information in just a glance, keeping the interface as uncluttered as possible.

## v0.1.0 - Core & Configuration
- [x] Initial System Metrics
  - [x] OS Name and Kernel Version
  - [x] Real-time CPU Usage & Clock Speed
  - [x] Real-time Memory (RAM) Usage
  - [x] Real-time Swap Usage
- [x] Basic UI
  - [x] Clean ANSI-colored terminal output
  - [x] Live 1-second interval updates with a hidden cursor
  - [x] Graceful exit handling (Ctrl+C)
- [ ] Modular Configuration
  - [ ] Allow users to explicitly enable/disable specific modules to minimize clutter.
  - [ ] Add support for customizing layout formats and colors.
  - [ ] Add support for a configuration file (e.g., `ssm.toml`).

## v0.2.0 - Minimal Expanded Metrics
- [ ] Optional, highly compact indicators for additional hardware:
  - [ ] Network usage (Up/Down speeds)
  - [ ] Disk space (Primary partition by default)
  - [ ] Hardware temperature sensors (CPU/GPU)
  - [ ] Battery status (Percentage and charging state)
- [ ] CLI arguments for quick overrides (e.g., `-n 2` for refresh rate).

## v0.3.0 - UI Polish & Focused Details
- [ ] Eliminate screen flicker using efficient terminal clearing or a lightweight TUI approach while preserving the simple CLI aesthetic.
- [ ] Optional "Top Process" view: A highly compact list of the top 3-5 CPU/Memory consuming processes, hidden by default to prevent clutter.

## v1.0.0 - Stability & Distribution
- [ ] Pre-defined, clean color themes out of the box.
- [ ] Automated cross-platform binary releases via GitHub Actions.
- [ ] Codebase optimization, ensuring minimal footprint and resource usage of the monitor itself.
