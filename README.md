# Prettier Process - PpsX

**PpsX** is a Rust command-line tool to explore and monitor Linux processes in a clean and readable way.
It supports both text and JSON output formats and includes a live monitoring mode.

## Features

- **List Processes**
Display all running processes in a simple way.

- **Process details**
Show more information about a specific process.

- **Statistics**
Provides an overview of processes and their statistics (such as impact on memory).

- **Live monitoring**
Track a process in real-time.

## Installation

**Make sure you have Rust**

- Install the files. (`git clone https://github.com/ZeIron16/Prettier-Process_PpsX.git`)

- You would then be able to use the commands from target/release/ppsx.

- In the main directory, use: `cargo build --release`

- Then you can simply use: `cargo install --path`

## Usage

### Main Commands

| Command | Description | Options |
|---------|-------------|---------|
| `list` | List all processes | `--json` for JSON output<br>`--file` to save to file in the current directory|
| `pinfo <PID>` | Details of a specific process | `--json` for JSON output<br> `--file` to save to file in the current directory|
| `stats` | System-wide process statistics | `--json` for JSON output<br>`--file` to save to file in the current directory|
| `live <PID>` | Real-time process monitoring | `--json` for JSON output<br>|

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `clap` | 4.3 | Command-line argument parsing |
| `serde` | 1.0 | Serialization framework |
| `serde_json` | 1.0 | JSON serialization |
| `libc` | 0.2 | System calls (CLK_TCK) |
| `chrono` | 0.4 | Date and time formatting |
