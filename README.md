# rust-tui-tasks

A lightweight, keyboard-driven task manager for your terminal, written in Rust.

Built with [Ratatui](https://github.com/ratatui/ratatui) and [Crossterm](https://github.com/crossterm-rs/crossterm).

## Features

- Keyboard-first navigation with arrow keys and Vim-style `j` / `k`
- Add, complete, and delete tasks without leaving the terminal
- Automatic JSON persistence after every change
- Unicode-aware input cursor positioning (including Persian text)
- Atomic file replacement to reduce the risk of partially written task data
- Protects an unreadable or invalid task file from being overwritten
- Restores terminal state after normal exits, runtime errors, and panics
- Event polling with redraws only when needed

## Preview

Tasks show their completion state at the end of each line:

```text
▶ Buy milk [ ]
  Write README [*]
```

## Requirements

- Rust stable (install with [rustup](https://rustup.rs))
- A terminal that supports alternate screen and raw mode

## Install and run

```bash
git clone https://github.com/Amirsadraiashgry/rust-tui-tasks.git
cd rust-tui-tasks
cargo run --release
```

To install the executable into Cargo's bin directory:

```bash
cargo install --path .
rust-tui-tasks
```

## Keybindings

| Key | Action |
| --- | --- |
| `Up` / `k` | Move selection up |
| `Down` / `j` | Move selection down |
| `Tab` | Toggle the selected task |
| `a` | Add a task |
| `d` | Delete the selected task |
| `q` / `Esc` | Quit |
| `Ctrl+C` | Quit |

While adding a task, press `Enter` to save or `Esc` to cancel.

## Data storage

Tasks are stored in `~/tasks.json`. If the home directory cannot be determined, the application uses `./tasks.json`.

The file is plain JSON and can be backed up or edited with a text editor. If an existing file cannot be read or parsed, the application displays an error and disables task mutations for that session. This is intentional: it prevents an empty in-memory list from replacing potentially recoverable data.

Writes use a temporary file in the same directory and rename it into place after the data has been written and synced. Keep regular backups of important task data.

## Development

Run the test suite:

```bash
cargo test
```

Check formatting:

```bash
cargo fmt --all -- --check
```

Run Clippy:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

## Project structure

```text
src/
├── main.rs      # terminal lifecycle, event loop, key handling
├── app.rs       # task model and application state
├── ui.rs        # terminal interface
└── storage.rs   # JSON loading and persistence
```

## License

Licensed under the [MIT License](LICENSE).
