# rust-tui-tasks

A fast, keyboard-driven task manager for your terminal, written in Rust with a blue-and-black theme.

Built with [ratatui](https://github.com/ratatui/ratatui) and [crossterm](https://github.com/crossterm-rs/crossterm).

## Features

- Full keyboard control, with vim-style `j` / `k` navigation
- Modern blue-on-black theme with a highlighted selection
- Tasks saved automatically after every change
- Low CPU usage (event polling, redraws only when needed)
- Terminal is restored cleanly on exit, even after a panic

Tasks are displayed with their status at the end of the line:

```
Buy milk [ ]
Write README [*]
```

## Installation

Requires [Rust](https://rustup.rs) (stable).

```bash
git clone https://github.com/USERNAME/rust-tui-tasks.git
cd rust-tui-tasks
cargo run --release
```

To install the binary globally:

```bash
cargo install --path .
rust-tui-tasks
```

## Keybindings

| Key              | Action                              |
| ---------------- | ----------------------------------- |
| `Up` / `k`       | Move selection up                   |
| `Down` / `j`     | Move selection down                 |
| `Tab`            | Toggle the selected task as done    |
| `a`              | Add a new task                      |
| `d`              | Delete the selected task            |
| `q` / `Esc`      | Quit                                |
| `Ctrl+C`         | Quit                                |

In input mode: `Enter` saves the task, `Esc` cancels.

## Data storage

Tasks are stored as JSON in `~/tasks.json` (or `./tasks.json` if the home directory can't be found). The file is human-readable and easy to back up:

```json
[
  { "id": 1, "text": "Buy milk", "done": false }
]
```

## Project structure

```
src/
├── main.rs      # terminal setup, event loop, key handling
├── app.rs       # App and Task structs, state logic
├── ui.rs        # rendering
└── storage.rs   # load/save JSON
```

## License

MIT, see `LICENSE` (add one before publishing if you want others to reuse the code).
