use rusqlite::{params, Connection, OptionalExtension, Result};
use std::path::Path;
use crate::task::Task;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        
        let db = Database { conn };
        db.init_schema()?;
        
        Ok(db)
    }

    fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                description TEXT,
                priority INTEGER NOT NULL DEFAULT 2,
                completed BOOLEAN NOT NULL DEFAULT 0,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                completed_at DATETIME,
                due_date DATE,
                category TEXT,
                tags TEXT
            );
            
            CREATE INDEX IF NOT EXISTS idx_completed ON tasks(completed);
            CREATE INDEX IF NOT EXISTS idx_priority ON tasks(priority);
            CREATE INDEX IF NOT EXISTS idx_due_date ON tasks(due_date);
            CREATE INDEX IF NOT EXISTS idx_category ON tasks(category);",
        )?;
        
        Ok(())
    }

    pub fn add_task(
        &self,
        title: &str,
        description: Option<&str>,
        priority: i32,
        due_date: Option<&str>,
        category: Option<&str>,
        tags: Option<&str>,
    ) -> Result<i32> {
        self.conn.execute(
            "INSERT INTO tasks (title, description, priority, due_date, category, tags) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![title, description, priority, due_date, category, tags],
        )?;
        
        Ok(self.conn.last_insert_rowid() as i32)
    }

    pub fn get_all_tasks(&self) -> Result<Vec<Task>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, description, priority, completed, created_at, completed_at, due_date, category, tags
             FROM tasks
             ORDER BY completed ASC, priority DESC, due_date ASC NULLS LAST, created_at ASC"
        )?;
        
        let tasks = stmt.query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                priority: row.get(3)?,
                completed: row.get(4)?,
                created_at: row.get(5)?,
                completed_at: row.get(6)?,
                due_date: row.get(7)?,
                category: row.get(8)?,
                tags: row.get(9)?,
            })
        })?;

        let mut result = Vec::new();
        for task in tasks {
            result.push(task?);
        }
        
        Ok(result)
    }

    pub fn get_task(&self, id: i32) -> Result<Option<Task>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, description, priority, completed, created_at, completed_at, due_date, category, tags
             FROM tasks WHERE id = ?1"
        )?;
        
        let task = stmt.query_row(params![id], |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                priority: row.get(3)?,
                completed: row.get(4)?,
                created_at: row.get(5)?,
                completed_at: row.get(6)?,
                due_date: row.get(7)?,
                category: row.get(8)?,
                tags: row.get(9)?,
            })
        }).optional()?;

        Ok(task)
    }

    pub fn search_tasks(&self, query: &str) -> Result<Vec<Task>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, description, priority, completed, created_at, completed_at, due_date, category, tags
             FROM tasks 
             WHERE title LIKE ?1 OR description LIKE ?1 OR tags LIKE ?1
             ORDER BY completed ASC, priority DESC, created_at ASC"
        )?;
        
        let search_query = format!("%{}%", query);
        let tasks = stmt.query_map([&search_query], |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                priority: row.get(3)?,
                completed: row.get(4)?,
                created_at: row.get(5)?,
                completed_at: row.get(6)?,
                due_date: row.get(7)?,
                category: row.get(8)?,
                tags: row.get(9)?,
            })
        })?;

        let mut result = Vec::new();
        for task in tasks {
            result.push(task?);
        }
        
        Ok(result)
    }

    pub fn get_tasks_by_category(&self, category: &str) -> Result<Vec<Task>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, description, priority, completed, created_at, completed_at, due_date, category, tags
             FROM tasks 
             WHERE category = ?1
             ORDER BY completed ASC, priority DESC, created_at ASC"
        )?;
        
        let tasks = stmt.query_map([category], |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                priority: row.get(3)?,
                completed: row.get(4)?,
                created_at: row.get(5)?,
                completed_at: row.get(6)?,
                due_date: row.get(7)?,
                category: row.get(8)?,
                tags: row.get(9)?,
            })
        })?;

        let mut result = Vec::new();
        for task in tasks {
            result.push(task?);
        }
        
        Ok(result)
    }

    pub fn get_overdue_tasks(&self) -> Result<Vec<Task>> {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let mut stmt = self.conn.prepare(
            "SELECT id, title, description, priority, completed, created_at, completed_at, due_date, category, tags
             FROM tasks 
             WHERE completed = 0 AND due_date < ?1
             ORDER BY due_date ASC"
        )?;
        
        let tasks = stmt.query_map([&today], |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                priority: row.get(3)?,
                completed: row.get(4)?,
                created_at: row.get(5)?,
                completed_at: row.get(6)?,
                due_date: row.get(7)?,
                category: row.get(8)?,
                tags: row.get(9)?,
            })
        })?;

        let mut result = Vec::new();
        for task in tasks {
            result.push(task?);
        }
        
        Ok(result)
    }

    pub fn complete_task(&self, id: i32) -> Result<()> {
        self.conn.execute(
            "UPDATE tasks SET completed = 1, completed_at = CURRENT_TIMESTAMP WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    pub fn delete_task(&self, id: i32) -> Result<()> {
        self.conn.execute(
            "DELETE FROM tasks WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    pub fn update_task(
        &self,
        id: i32,
        title: Option<&str>,
        description: Option<&str>,
        priority: Option<i32>,
        due_date: Option<&str>,
        category: Option<&str>,
        tags: Option<&str>,
    ) -> Result<()> {
        let mut query = "UPDATE tasks SET ".to_string();
        let mut updates = Vec::new();
        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(t) = title {
            updates.push("title = ?");
            params_vec.push(Box::new(t.to_string()));
        }
        if let Some(d) = description {
            updates.push("description = ?");
            params_vec.push(Box::new(d.to_string()));
        }
        if let Some(p) = priority {
            updates.push("priority = ?");
            params_vec.push(Box::new(p));
        }
        if let Some(dd) = due_date {
            updates.push("due_date = ?");
            params_vec.push(Box::new(dd.to_string()));
        }
        if let Some(c) = category {
            updates.push("category = ?");
            params_vec.push(Box::new(c.to_string()));
        }
        if let Some(tg) = tags {
            updates.push("tags = ?");
            params_vec.push(Box::new(tg.to_string()));
        }

        if updates.is_empty() {
            return Ok(());
        }

        query.push_str(&updates.join(", "));
        query.push_str(" WHERE id = ?");
        params_vec.push(Box::new(id));

        let mut stmt = self.conn.prepare(&query)?;
        let params_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();
        stmt.execute(rusqlite::params_from_iter(params_refs))?;

        Ok(())
    }

    pub fn get_stats(&self) -> Result<(i32, i32, i32)> {
        let total: i32 = self.conn.query_row(
            "SELECT COUNT(*) FROM tasks",
            [],
            |row| row.get(0)
        )?;

        let completed: i32 = self.conn.query_row(
            "SELECT COUNT(*) FROM tasks WHERE completed = 1",
            [],
            |row| row.get(0)
        )?;

        let pending: i32 = self.conn.query_row(
            "SELECT COUNT(*) FROM tasks WHERE completed = 0",
            [],
            |row| row.get(0)
        )?;

        Ok((total, completed, pending))
    }

    pub fn get_categories(&self) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT DISTINCT category FROM tasks WHERE category IS NOT NULL ORDER BY category"
        )?;
        
        let categories = stmt.query_map([], |row| row.get(0))?;
        let mut result = Vec::new();
        for cat in categories {
            result.push(cat?);
        }
        
        Ok(result)
    }
}
