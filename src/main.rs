mod db;
mod task;
mod commands;
mod tui;

use clap::Parser;
use std::path::PathBuf;
use std::io;

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A powerful CLI todo manager for Linux", long_about = None)]
#[command(version)]
#[command(author = "")]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Path to the database file (defaults to ~/.local/share/todo/tasks.db)
    #[arg(global = true, long)]
    db: Option<PathBuf>,

    /// Launch interactive TUI mode
    #[arg(short, long)]
    interactive: bool,
}

#[derive(Parser)]
enum Commands {
    /// Add a new task
    Add {
        /// Task title
        title: String,

        /// Task description (optional)
        #[arg(short, long)]
        description: Option<String>,

        /// Priority level: 1 (low), 2 (medium), 3 (high)
        #[arg(short, long, default_value = "2")]
        priority: i32,

        /// Due date (YYYY-MM-DD format)
        #[arg(long)]
        due: Option<String>,

        /// Task category
        #[arg(short, long)]
        category: Option<String>,
    },

    /// List all tasks
    List {
        /// Show only completed tasks
        #[arg(short, long)]
        completed: bool,

        /// Show only pending tasks
        #[arg(short, long)]
        pending: bool,

        /// Filter by priority (1-3)
        #[arg(short, long)]
        priority: Option<i32>,

        /// Filter by category
        #[arg(short, long)]
        category: Option<String>,
    },

    /// Mark a task as complete
    Complete {
        /// Task ID
        id: i32,
    },

    /// Delete a task
    Delete {
        /// Task ID
        id: i32,
    },

    /// Edit a task
    Edit {
        /// Task ID
        id: i32,

        /// New title
        #[arg(short, long)]
        title: Option<String>,

        /// New description
        #[arg(short, long)]
        description: Option<String>,

        /// New priority (1-3)
        #[arg(short, long)]
        priority: Option<i32>,

        /// New due date (YYYY-MM-DD format)
        #[arg(long)]
        due: Option<String>,

        /// New category
        #[arg(short, long)]
        category: Option<String>,
    },

    /// Show task statistics
    Stats,

    /// Show overdue tasks
    Overdue,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let db_path = args.db.unwrap_or_else(|| {
        let mut path = dirs::home_dir().expect("Could not find home directory");
        path.push(".local/share/todo");
        path.push("tasks.db");
        path
    });

    // Ensure database directory exists
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create database directory");
    }

    let db = db::Database::new(&db_path).expect("Failed to initialize database");

    // If no command is provided or --interactive flag is set, launch TUI
    if args.command.is_none() || args.interactive {
        tui::run(&db)?;
        return Ok(());
    }

    // Otherwise, run CLI mode
    match args.command.unwrap() {
        Commands::Add {
            title,
            description,
            priority,
            due,
            category,
        } => {
            commands::add(&db, &title, description, priority, due, category)
                .expect("Failed to add task");
        }
        Commands::List {
            completed,
            pending,
            priority,
            category,
        } => {
            commands::list(&db, completed, pending, priority, category)
                .expect("Failed to list tasks");
        }
        Commands::Complete { id } => {
            commands::complete(&db, id).expect("Failed to complete task");
        }
        Commands::Delete { id } => {
            commands::delete(&db, id).expect("Failed to delete task");
        }
        Commands::Edit {
            id,
            title,
            description,
            priority,
            due,
            category,
        } => {
            commands::edit(&db, id, title, description, priority, due, category)
                .expect("Failed to edit task");
        }
        Commands::Stats => {
            commands::stats(&db).expect("Failed to show statistics");
        }
        Commands::Overdue => {
            commands::overdue(&db).expect("Failed to show overdue tasks");
        }
    }

    Ok(())
}

// Helper module for directory detection
mod dirs {
    use std::path::PathBuf;

    pub fn home_dir() -> Option<PathBuf> {
        std::env::var_os("HOME").and_then(|h| {
            if h.is_empty() {
                None
            } else {
                Some(h.into())
            }
        })
    }
}
