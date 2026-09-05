# 🚀 Quick Start Guide

## Installation (3 Steps)

### Step 1: Build

```bash
cargo build --release
```

The binary will be at: `target/release/todo`

### Step 2: Install

```bash
sudo mv target/release/todo /usr/local/bin/
```

Or add to PATH:
```bash
export PATH="$PATH:$(pwd)/target/release"
```

### Step 3: Run

```bash
todo
```

---

## First Steps

### Add Your First Task

```bash
# Simple task
todo add "Buy groceries"

# Task with priority and due date
todo add "Complete project" -p 3 --due 2024-12-31

# Task with category
todo add "Learn Rust" -c Learning
```

### Launch Interactive Mode

```bash
todo
```

### Quick Keyboard Reference

| Key | Action |
|-----|--------|
| `↑↓` | Navigate |
| `n` | New task |
| `e` | Edit |
| `Space` | Complete |
| `d` | Delete |
| `s` | Stats |
| `?` | Help |
| `q` | Quit |

---

## Common Tasks

### Check What's Due Today
```bash
# In TUI: press '4'
todo
```

### See All Overdue Tasks
```bash
todo overdue
```

### View Statistics
```bash
# In TUI: press 's'
todo stats
```

### Search Tasks
```bash
# In TUI: press '/'
# Or use list with filters:
todo list --category Work
```

### Mark Task as Done
```bash
# In CLI
todo complete 1

# In TUI
# Navigate to task and press Space
```

---

## Tips & Tricks

### Set Due Date Format
Always use: `YYYY-MM-DD`
```bash
# Today
todo add "Task" --due $(date +%Y-%m-%d)

# 7 days from now
due=$(date -d "+7 days" +%Y-%m-%d)
todo add "Task" --due $due
```

### Organize by Category
```bash
todo add "Meeting" -c Work
todo add "Exercise" -c Health
todo add "Read" -c Learning

# List by category
todo list --category Work
```

### Priority Quick Reference
- `1` = Low 🟢 (nice to have)
- `2` = Medium 🟡 (should do)
- `3` = High 🔴 (must do)

### Shell Alias
```bash
# Add to ~/.bashrc or ~/.zshrc
alias td='todo'
alias tdd='todo'  # typo-safe

# Then use:
td add "Quick task"
td list
```

### Daily Routine
```bash
# Every morning
todo  # Launch TUI

# Press '4' to see tasks due today
# Complete them with Space
# Press 's' to check progress
# Press 'q' to quit
```

---

## Database Location

Default: `~/.local/share/todo/tasks.db`

Custom location:
```bash
todo --db ~/my-tasks.db add "Task"
```

### Backup Your Tasks
```bash
cp ~/.local/share/todo/tasks.db ~/backup-tasks.db
```

---

## Troubleshooting

### "Permission denied" when installing
```bash
# Use sudo
sudo mv target/release/todo /usr/local/bin/

# Or install to user directory
mkdir -p ~/.local/bin
mv target/release/todo ~/.local/bin/
export PATH="$PATH:$HOME/.local/bin"
```

### Terminal looks weird
```bash
export TERM=xterm-256color
todo
```

### Database errors
```bash
# Remove corrupted database
rm ~/.local/share/todo/tasks.db

# Will be recreated on next run
todo list
```

### Build fails
```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

---

## Next Steps

1. ✅ Install the application
2. ✅ Add a few test tasks
3. ✅ Learn keyboard shortcuts (press `?` in TUI)
4. ✅ Create tasks with due dates
5. ✅ Organize by categories
6. ✅ Check daily statistics
7. ✅ Integrate into your workflow

---

## Need Help?

- **Press `?`** in TUI for keyboard shortcuts
- **Read README.md** for detailed documentation
- **Check IMPROVEMENTS.md** for feature details
- **Run `todo --help`** for CLI help

---

## You're Ready! 🎉

Start using your powerful task manager:

```bash
todo
```

Enjoy! 🚀
