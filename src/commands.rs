use crate::db::Database;
use colored::*;
use rusqlite::Result;

fn print_header(title: &str) {
    println!("{}", "╔══════════════════════════════════════════════════════╗".cyan().bold());
    println!("{} {:^52} {}",
        "║".cyan().bold(),
        title.white().bold(),
        "║".cyan().bold()
    );
    println!("{}", "╚══════════════════════════════════════════════════════╝".cyan().bold());
}

pub fn add(
    db: &Database,
    title: &str,
    description: Option<String>,
    priority: i32,
    due_date: Option<String>,
    category: Option<String>,
) -> Result<()> {
    if priority < 1 || priority > 3 {
        eprintln!("{}Priority must be 1 (low), 2 (medium), or 3 (high)", "error: ".red().bold());
        std::process::exit(1);
    }

    let id = db.add_task(
        title,
        description.as_deref(),
        priority,
        due_date.as_deref(),
        category.as_deref(),
        None,
    )?;
    
    println!();
    print_header("✨ Task Created");
    println!("  {} {}",
        "ID:".green().bold(),
        id.to_string().cyan()
    );
    println!("  {} {}",
        "Title:".green().bold(),
        title
    );
    if let Some(d) = &description {
        println!("  {} {}",
            "Description:".green().bold(),
            d
        );
    }
    println!("  {} {}",
        "Priority:".green().bold(),
        match priority {
            1 => "Low 🟢".green(),
            2 => "Medium 🟡".yellow(),
            3 => "High 🔴".red(),
            _ => "Unknown".normal(),
        }
    );
    if let Some(dd) = &due_date {
        println!("  {} {}",
            "Due:".green().bold(),
            dd
        );
    }
    if let Some(cat) = &category {
        println!("  {} {}",
            "Category:".green().bold(),
            cat
        );
    }
    println!();
    Ok(())
}

pub fn list(
    db: &Database,
    only_completed: bool,
    only_pending: bool,
    priority_filter: Option<i32>,
    category_filter: Option<String>,
) -> Result<()> {
    let tasks = db.get_all_tasks()?;

    let filtered: Vec<_> = tasks
        .into_iter()
        .filter(|t| {
            if only_completed && !t.completed {
                return false;
            }
            if only_pending && t.completed {
                return false;
            }
            if let Some(p) = priority_filter {
                if t.priority != p {
                    return false;
                }
            }
            if let Some(ref cat) = category_filter {
                if t.category.as_ref() != Some(cat) {
                    return false;
                }
            }
            true
        })
        .collect();

    if filtered.is_empty() {
        println!("{}", "\n No tasks found.\n".yellow());
        return Ok(());
    }

    println!();
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".cyan());
    println!(
        "{:>3} {} {:<35} {:<12} {:<18} {}",
        "ID".bold().cyan(),
        "●".bold().cyan(),
        "Title".bold().cyan(),
        "Priority".bold().cyan(),
        "Due".bold().cyan(),
        "Category".bold().cyan(),
    );
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".cyan());

    for task in &filtered {
        let status = if task.completed {
            "✓".green().bold()
        } else if task.is_overdue() {
            "⚠".red().bold()
        } else {
            "○".yellow().bold()
        };

        let priority_str = match task.priority {
            1 => "Low 🟢".green(),
            2 => "Medium 🟡".yellow(),
            3 => "High 🔴".red().bold(),
            _ => "?".normal(),
        };

        let title = if task.completed {
            task.title.strikethrough().dimmed()
        } else {
            task.title.normal()
        };

        let due_str = if let Some(due) = task.format_due_date() {
            if task.is_overdue() {
                due.red().to_string()
            } else if task.is_due_today() {
                due.yellow().to_string()
            } else {
                due.normal().to_string()
            }
        } else {
            "-".dimmed().to_string()
        };

        let category = task.category.as_ref().unwrap_or(&"-".to_string()).to_string();

        println!(
            "{:>3} {} {:<35} {:<12} {:<18} {}",
            task.id.to_string().cyan(),
            status,
            title,
            priority_str,
            due_str,
            category
        );

        if let Some(desc) = &task.description {
            println!("     {}", desc.dimmed());
        }
    }

    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".cyan());
    println!(
        "{} {} {}",
        "Showing".dimmed(),
        filtered.len().to_string().cyan().bold(),
        "task(s)".dimmed()
    );
    println!();

    Ok(())
}

pub fn complete(db: &Database, id: i32) -> Result<()> {
    match db.get_task(id)? {
        Some(task) => {
            db.complete_task(id)?;
            println!();
            println!(
                "{} {}",
                "✓ Completed:".green().bold(),
                task.title.cyan()
            );
            println!();
            Ok(())
        }
        None => {
            eprintln!("{}Task with ID {} not found", "error: ".red().bold(), id);
            std::process::exit(1);
        }
    }
}

pub fn delete(db: &Database, id: i32) -> Result<()> {
    match db.get_task(id)? {
        Some(task) => {
            db.delete_task(id)?;
            println!();
            println!(
                "{} {}",
                "✓ Deleted:".red().bold(),
                task.title.cyan()
            );
            println!();
            Ok(())
        }
        None => {
            eprintln!("{}Task with ID {} not found", "error: ".red().bold(), id);
            std::process::exit(1);
        }
    }
}

pub fn edit(
    db: &Database,
    id: i32,
    title: Option<String>,
    description: Option<String>,
    priority: Option<i32>,
    due_date: Option<String>,
    category: Option<String>,
) -> Result<()> {
    match db.get_task(id)? {
        Some(_task) => {
            if let Some(p) = priority {
                if p < 1 || p > 3 {
                    eprintln!("{}Priority must be 1 (low), 2 (medium), or 3 (high)", "error: ".red().bold());
                    std::process::exit(1);
                }
            }

            db.update_task(
                id,
                title.as_deref(),
                description.as_deref(),
                priority,
                due_date.as_deref(),
                category.as_deref(),
                None,
            )?;
            println!();
            println!("{}", "✓ Task updated successfully".green().bold());
            println!();
            Ok(())
        }
        None => {
            eprintln!("{}Task with ID {} not found", "error: ".red().bold(), id);
            std::process::exit(1);
        }
    }
}

pub fn stats(db: &Database) -> Result<()> {
    let (total, completed, pending) = db.get_stats()?;
    let overdue = db.get_overdue_tasks()?.len();

    let completion_rate = if total > 0 {
        (completed as f32 / total as f32) * 100.0
    } else {
        0.0
    };

    println!();
    println!("{}", "╔════════════════════════════════════════╗".cyan().bold());
    println!(
        "{} {} {}",
        "║ ".cyan().bold(),
        "📊 Task Statistics".bold(),
        "║".cyan().bold()
    );
    println!("{}", "╠════════════════════════════════════════╣".cyan().bold());
    println!(
        "{} Total Tasks:        {:>3}  {}",
        "║ ".cyan().bold(),
        total.to_string().cyan().bold(),
        "║".cyan().bold()
    );
    println!(
        "{} Completed:          {:>3}  {}",
        "║ ".cyan().bold(),
        completed.to_string().green().bold(),
        "║".cyan().bold()
    );
    println!(
        "{} Pending:            {:>3}  {}",
        "║ ".cyan().bold(),
        pending.to_string().yellow().bold(),
        "║".cyan().bold()
    );
    println!(
        "{} Overdue:            {:>3}  {}",
        "║ ".cyan().bold(),
        overdue.to_string().red().bold(),
        "║".cyan().bold()
    );
    println!(
        "{} Completion Rate:  {:.1}%  {}",
        "║ ".cyan().bold(),
        completion_rate,
        "║".cyan().bold()
    );
    println!("{}", "╚════════════════════════════════════════╝".cyan().bold());
    println!();

    Ok(())
}

pub fn overdue(db: &Database) -> Result<()> {
    let tasks = db.get_overdue_tasks()?;

    if tasks.is_empty() {
        println!("{}", "\n✓ No overdue tasks!\n".green().bold());
        return Ok(());
    }

    println!();
    println!("{}", "⚠️  OVERDUE TASKS".red().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".red());

    for task in &tasks {
        println!(
            "{} {} {} - {}",
            "ID".bold().red(),
            task.id.to_string().red().bold(),
            task.title.red(),
            task.format_due_date().unwrap_or_default().red().bold()
        );
    }

    println!();

    Ok(())
}