use serde::{Serialize, Deserialize};
use chrono::{Local, NaiveDate};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub priority: i32,
    pub completed: bool,
    pub created_at: String,
    pub completed_at: Option<String>,
    pub due_date: Option<String>,
    pub category: Option<String>,
    pub tags: Option<String>,
}

impl Task {
    pub fn priority_label(&self) -> &'static str {
        match self.priority {
            1 => "Low",
            2 => "Medium",
            3 => "High",
            _ => "Unknown",
        }
    }

    pub fn priority_emoji(&self) -> &'static str {
        match self.priority {
            1 => "🟢",
            2 => "🟡",
            3 => "🔴",
            _ => "⚪",
        }
    }

    pub fn status_label(&self) -> &'static str {
        if self.completed {
            "✓"
        } else {
            "○"
        }
    }

    pub fn is_overdue(&self) -> bool {
        if self.completed {
            return false;
        }
        
        if let Some(due) = &self.due_date {
            if let Ok(due_date) = NaiveDate::parse_from_str(due, "%Y-%m-%d") {
                let today = Local::now().naive_local().date();
                return due_date < today;
            }
        }
        false
    }

    pub fn is_due_today(&self) -> bool {
        if self.completed {
            return false;
        }
        
        if let Some(due) = &self.due_date {
            if let Ok(due_date) = NaiveDate::parse_from_str(due, "%Y-%m-%d") {
                let today = Local::now().naive_local().date();
                return due_date == today;
            }
        }
        false
    }

    pub fn days_until_due(&self) -> Option<i64> {
        if let Some(due) = &self.due_date {
            if let Ok(due_date) = NaiveDate::parse_from_str(due, "%Y-%m-%d") {
                let today = Local::now().naive_local().date();
                let duration = due_date.signed_duration_since(today);
                return Some(duration.num_days());
            }
        }
        None
    }

    pub fn format_due_date(&self) -> Option<String> {
        if let Some(due) = &self.due_date {
            if let Ok(due_date) = NaiveDate::parse_from_str(due, "%Y-%m-%d") {
                let today = Local::now().naive_local().date();
                
                let days_diff = due_date.signed_duration_since(today).num_days();
                
                return Some(match days_diff {
                    -1000..=-2 => format!("{} days ago", -days_diff),
                    -1 => "Yesterday".to_string(),
                    0 => "Today".to_string(),
                    1 => "Tomorrow".to_string(),
                    2..=30 => format!("In {} days", days_diff),
                    _ => due_date.format("%b %d, %Y").to_string(),
                });
            }
        }
        None
    }

    pub fn get_tags_list(&self) -> Vec<String> {
        self.tags
            .as_ref()
            .map(|t| t.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default()
    }
}