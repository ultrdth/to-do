# 📋 Task Master - Advanced CLI Todo Manager

> A powerful, fast, and beautiful CLI todo/task manager for Linux built with Rust and SQLite.

![License](https://img.shields.io/badge/license-MIT-green.svg)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)

## ✨ Features

### Core Features
- ✅ **Interactive TUI Mode** - Full-featured terminal UI with smooth navigation
- 💻 **CLI Mode** - Classic command-line interface for scripting and automation
- 🎯 **Priority Levels** - Low, Medium, High priority tasks with visual indicators
- 📅 **Due Dates** - Set due dates and track deadline-based tasks
- 🏷️ **Categories** - Organize tasks by custom categories
- 📝 **Descriptions** - Add detailed descriptions to tasks
- 🔍 **Search** - Full-text search across titles, descriptions, and tags
- 📊 **Statistics** - Visual progress gauges and detailed statistics
- ⚠️ **Overdue Tracking** - Highlight and manage overdue tasks
- 🎨 **Beautiful Terminal UI** - Color-coded output with emojis and ASCII art

### Smart Filtering
- Filter by status (pending, completed)
- Filter by priority level
- Filter by due date (today, overdue)
- Filter by category
- Search with intelligent matching

### Advanced Features
- Due date intelligence (shows "Today", "Tomorrow", "In N days", etc.)
- Overdue task warnings with visual indicators
- Task completion tracking with timestamps
- Category-based organization
- Bulk operations
- Beautiful ASCII-art statistics dashboard

## 🎮 Installation

### Prerequisites

- **Rust 1.70+** ([Install from rustup.rs](https://rustup.rs/))
- **Linux** (Ubuntu, Arch, Fedora, etc.)

### Build from Source

```bash
# Clone the repository
git clone [https://github.com/ultrdth/to-do.git](https://github.com/ultrdth/to-do.git)
cd to-do

# Build the project
cargo build --release

# The binary will be at: target/release/todo

```

### Install to PATH

```bash
# Option 1: Move to system bin
sudo mv target/release/todo /usr/local/bin/

# Option 2: Add to PATH in ~/.bashrc or ~/.zshrc
export PATH="$PATH:$(pwd)/target/release"

```

## 🚀 Usage

### Interactive TUI Mode

**Launch the interactive terminal interface:**

```bash
todo              # Launch TUI
todo -i           # Alternative: use --interactive flag

```

### Keyboard Shortcuts

#### Navigation

| Key | Action |
| --- | --- |
| `↑` / `↓` | Navigate tasks |
| `Enter` | View full task details |
| `Esc` | Back to list |

#### Task Management

| Key | Action |
| --- | --- |
| `n` | Add new task |
| `e` | Edit task |
| `Space` | Toggle task completion |
| `d` | Delete task |

#### Filtering & Views

| Key | Action |
| --- | --- |
| `0` | Show all tasks |
| `1` | Show pending tasks only |
| `2` | Show completed tasks |
| `3` | Show overdue tasks ⚠️ |
| `4` | Show tasks due today 📅 |
| `/` | Search tasks |

#### Other

| Key | Action |
| --- | --- |
| `s` | View statistics 📊 |
| `?` | Show help |
| `q` | Quit |

---

### CLI Mode

#### Add a Task

```bash
# Simple task
todo add "Buy groceries"

# With all details
todo add "Finish project" \
  -d "Complete the REST API implementation" \
  -p 3 \
  --due 2026-12-25 \
  -c "Work"

# Arguments:
# title              Task title (required)
# -d, --description  Task description
# -p, --priority     1=Low, 2=Medium, 3=High (default: 2)
# --due              Due date (YYYY-MM-DD format)
# -c, --category     Task category

```

#### List Tasks

```bash
# List all pending tasks (default)
todo list

# Show only pending tasks
todo list --pending

# Show completed tasks
todo list --completed

# Filter by priority
todo list --priority 3  # Only high priority

# Filter by category
todo list --category Work

```

#### Complete a Task

```bash
todo complete 1  # Mark task ID 1 as complete

```

#### Delete a Task

```bash
todo delete 1  # Delete task ID 1

```

#### Edit a Task

```bash
# Edit title
todo edit 1 --title "Updated title"

# Edit multiple fields
todo edit 1 \
  --title "New title" \
  -d "New description" \
  -p 3 \
  --due 2026-12-31 \
  -c "Personal"

```

#### View Statistics

```bash
todo stats  # Show completion rate, overdue count, etc.

```

#### Show Overdue Tasks

```bash
todo overdue  # Display all overdue tasks with warnings

```

#### Custom Database Location

```bash
todo --db /path/to/custom.db list
todo --db ~/my-todos.db add "Task"

```

## 📊 Database Schema

The application uses SQLite with the following schema:

```sql
CREATE TABLE tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT,
    priority INTEGER NOT NULL DEFAULT 2,
    completed BOOLEAN NOT NULL DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    completed_at DATETIME,
    due_date DATE,
    category TEXT,
    tags TEXT  -- comma-separated tags
);

```

**Indexes:**

* `idx_completed` - Fast filtering by completion status
* `idx_priority` - Fast filtering by priority
* `idx_due_date` - Fast sorting/filtering by due date
* `idx_category` - Fast category filtering

## 🎨 Visual Indicators

### Priority Emojis

* 🟢 Low
* 🟡 Medium
* 🔴 High

### Status Indicators

* `●` Pending task
* `✓` Completed task
* `⚠` Overdue task
* `○` In list view

### Due Date Display

* `Today` - Due today
* `Tomorrow` - Due tomorrow
* `In N days` - Future date
* `N days ago` - Overdue (shown in red)

## 📁 Project Structure

```
to-do/
├── src/
│   ├── main.rs       # CLI parsing & entry point
│   ├── db.rs         # SQLite database operations
│   ├── task.rs       # Task data structure & methods
│   ├── commands.rs   # CLI command handlers
│   └── tui.rs        # Terminal UI implementation
├── Cargo.toml        # Dependencies
├── Cargo.lock        # Locked versions
└── README.md         # This file

```

## 📦 Dependencies

```toml
[dependencies]
rusqlite = "0.30"      # SQLite bindings
clap = "4.4"          # CLI argument parsing
colored = "2.1"       # Terminal colors
chrono = "0.4"        # Date/time handling
serde = "1.0"          # Serialization
ratatui = "0.27"      # TUI framework
crossterm = "0.27"    # Terminal control
dirs = "5.0"          # Directory utilities

```

## 🔧 Configuration

The default database location is:

```
~/.local/share/todo/tasks.db

```

This directory is created automatically if it doesn't exist.

### Custom Database

You can use any location by passing `--db` flag:

```bash
export TODO_DB=/tmp/my-tasks.db
todo --db $TODO_DB list

```

## 🎯 Workflow Examples

### Getting Started

```bash
# Add some tasks
todo add "Learn Rust" -p 2
todo add "Build project" -p 3 --due 2026-12-31 -c Work
todo add "Read documentation" -d "Read The Rust Book" -c Learning

# View all tasks
todo list

# Launch interactive mode
todo

```

### Daily Workflow

```bash
# Check what's due today
# (Press '4' in TUI mode)

# Complete a task
todo complete 5

# See how you're doing
todo stats

# Check overdue tasks
todo overdue

```

### Project Management

```bash
# List all work tasks
todo list --category Work

# Add sprint task with due date
todo add "API endpoint" -p 3 --due 2026-12-22 -c Work

# Mark as complete
todo complete 8

# View work progress
todo list --category Work

```

## 🚀 Roadmap / Future Enhancements

* [ ] Recurring tasks (daily, weekly, monthly)
* [ ] Task time tracking
* [ ] Export to JSON/CSV/iCalendar
* [ ] Task tags with advanced filtering
* [ ] Subtasks/nested tasks
* [ ] Shell completions (bash, zsh, fish)
* [ ] Config file support (.todorc)
* [ ] Undo/redo functionality
* [ ] Task notes/changelog
* [ ] Multi-user support
* [ ] Cloud sync (optional)
* [ ] Web dashboard (optional)

## 🧪 Development

### Run Tests

```bash
cargo test

```

### Format Code

```bash
cargo fmt

```

### Lint Code

```bash
cargo clippy -- -D warnings

```

### Build Documentation

```bash
cargo doc --open

```

### Build Release Binary

```bash
cargo build --release -j $(nproc)

```

## ⚡ Performance

* **Lightweight:** Minimal memory footprint (~2-5MB)
* **Fast:** SQLite queries optimized with proper indexing
* **Responsive:** TUI with 250ms event polling
* **No external dependencies:** Everything bundled

## 🐛 Troubleshooting

### Database locked error

```bash
# Delete corrupted database and start fresh
rm ~/.local/share/todo/tasks.db

# Or use custom location
todo --db /tmp/tasks.db add "Test"

```

### Terminal rendering issues

* Ensure terminal supports 256 colors
* Try exporting: `export TERM=xterm-256color`
* Update terminal emulator to latest version

### Build errors

```bash
# Update Rust
rustup update

# Clean build
cargo clean
cargo build --release

```

## 📝 License

MIT License - Use this project however you like!

```
Copyright (c) 2026

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software.

```

## 🤝 Contributing

Contributions are very welcome! Here's how:

1. **Fork** the repository
2. **Create** a feature branch: `git checkout -b feature/amazing-feature`
3. **Commit** your changes: `git commit -m 'Add amazing feature'`
4. **Push** to the branch: `git push origin feature/amazing-feature`
5. **Open** a Pull Request

### Guidelines

* Follow the existing code style
* Add tests for new features
* Update documentation
* Keep commits atomic and descriptive

## 💬 Support

* **Issues:** Report bugs on GitHub
* **Discussions:** Ask questions and share ideas
* **Pull Requests:** Submit improvements

---

**Made with ❤️ in Rust** 🦀

Built for productivity. Made for power users.

Happy task managing! 🚀

```

```
