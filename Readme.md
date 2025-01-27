

# Note Link Creator

A fast and efficient note management tool with full-text search capabilities and a modern GUI interface.

## Features

- 🔍 Full-text search across notes
- 🎯 Title-specific fuzzy search using BM25 trigram algorithm
- ⌨️ Keyboard-first interface for rapid navigation
- 🖥️ Clean, minimal GUI
- 📝 CLI support for automation and scripting
- 📋 Quick copy/paste functionality
- 🔗 Markdown-compatible link generation

## Installation

```bash
cargo install --path .
```

## Usage

### GUI Mode

Launch the application in GUI mode:
```bash
noteapp -d path/to/database.db
```

#### Keyboard Shortcuts
- `Ctrl+S`: Focus title filter
- `Ctrl+B`: Focus body filter
- `Ctrl+L`: Focus list
- `Ctrl+C`: Copy selected note
- `Ctrl+N`: Next item
- `Ctrl+P`: Previous item
- `Enter`: Generate link and exit
- `j/↓`: Move selection down
- `k/↑`: Move selection up

### CLI Mode

List all notes:
```bash
noteapp -d path/to/database.db list
```

Search notes:
```bash
noteapp -d path/to/database.db search "query"
```

Preview note content:
```bash
noteapp -d path/to/database.db preview "note-id"
```

## Database Schema

The application expects an SQLite database with a `notes` table containing:
- `id`: Unique identifier
- `title`: Note title
- `body`: Note content

## Development

Built with:
- [egui](https://github.com/emilk/egui) - Pure Rust GUI framework
- [rusqlite](https://github.com/rusqlite/rusqlite) - SQLite bindings for Rust
- [clap](https://github.com/clap-rs/clap) - Command line argument parser

## License

[MIT License](LICENSE)
