use crate::db::Database;
use crate::task::Task;
use colored::Colorize;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};
use std::io;

#[derive(Clone, Copy, Debug)]
enum Screen {
    List,
    AddTask,
    EditTask,
    TaskDetail,
    Stats,
    Search,
    Help,
}

#[derive(Clone, Copy, Debug)]
enum ListFilter {
    All,
    Pending,
    Completed,
    Overdue,
    Today,
}

pub struct App<'a> {
    db: &'a Database,
    tasks: Vec<Task>,
    filtered_tasks: Vec<Task>,
    selected: usize,
    screen: Screen,
    input_buffer: String,
    editing_id: Option<i32>,
    editing_field: EditField,
    filter: ListFilter,
    show_help: bool,
    detail_scroll: usize,
    pub should_quit: bool,
}

#[derive(Clone, Copy, Debug)]
enum EditField {
    Title,
    Description,
    DueDate,
    Priority,
    Category,
}

impl<'a> App<'a> {
    fn new(db: &'a Database) -> io::Result<Self> {
        let tasks = db.get_all_tasks().expect("Failed to fetch tasks");
        let mut app = App {
            db,
            filtered_tasks: tasks.clone(),
            tasks,
            selected: 0,
            screen: Screen::List,
            input_buffer: String::new(),
            editing_id: None,
            editing_field: EditField::Title,
            filter: ListFilter::All,
            show_help: false,
            detail_scroll: 0,
            should_quit: false,
        };
        app.apply_filter();
        Ok(app)
    }

    fn apply_filter(&mut self) {
        self.filtered_tasks = self.tasks.iter()
            .filter(|t| match self.filter {
                ListFilter::All => !t.completed,
                ListFilter::Pending => !t.completed,
                ListFilter::Completed => t.completed,
                ListFilter::Overdue => t.is_overdue(),
                ListFilter::Today => t.is_due_today(),
            })
            .cloned()
            .collect();

        if self.selected >= self.filtered_tasks.len() && !self.filtered_tasks.is_empty() {
            self.selected = self.filtered_tasks.len() - 1;
        } else if self.filtered_tasks.is_empty() {
            self.selected = 0;
        }
    }

    fn refresh_tasks(&mut self) {
        self.tasks = self.db.get_all_tasks().expect("Failed to fetch tasks");
        self.apply_filter();
    }

    fn handle_key(&mut self, key: KeyCode) {
        if self.show_help {
            if key == KeyCode::Char('?') || key == KeyCode::Esc {
                self.show_help = false;
            }
            return;
        }

        match self.screen {
            Screen::List => self.handle_list_key(key),
            Screen::AddTask => self.handle_add_key(key),
            Screen::EditTask => self.handle_edit_key(key),
            Screen::TaskDetail => self.handle_detail_key(key),
            Screen::Stats => self.handle_stats_key(key),
            Screen::Search => self.handle_search_key(key),
            Screen::Help => self.handle_help_key(key),
        }
    }

    fn handle_list_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Char('?') => self.show_help = true,
            KeyCode::Char('n') => {
                self.screen = Screen::AddTask;
                self.input_buffer.clear();
                self.editing_field = EditField::Title;
            }
            KeyCode::Char('/') => {
                self.screen = Screen::Search;
                self.input_buffer.clear();
            }
            KeyCode::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            }
            KeyCode::Down => {
                if self.selected < self.filtered_tasks.len().saturating_sub(1) {
                    self.selected += 1;
                }
            }
            KeyCode::Char(' ') => {
                if let Some(task) = self.filtered_tasks.get(self.selected) {
                    let _ = self.db.complete_task(task.id);
                    self.refresh_tasks();
                }
            }
            KeyCode::Char('d') => {
                if let Some(task) = self.filtered_tasks.get(self.selected) {
                    let _ = self.db.delete_task(task.id);
                    self.refresh_tasks();
                }
            }
            KeyCode::Char('e') => {
                if let Some(task) = self.filtered_tasks.get(self.selected) {
                    self.editing_id = Some(task.id);
                    self.screen = Screen::EditTask;
                    self.input_buffer = task.title.clone();
                    self.editing_field = EditField::Title;
                }
            }
            KeyCode::Enter => {
                if self.filtered_tasks.get(self.selected).is_some() {
                    self.screen = Screen::TaskDetail;
                    self.detail_scroll = 0;
                }
            }
            KeyCode::Char('s') => {
                self.screen = Screen::Stats;
            }
            KeyCode::Char('1') => {
                self.filter = ListFilter::Pending;
                self.selected = 0;
                self.apply_filter();
            }
            KeyCode::Char('2') => {
                self.filter = ListFilter::Completed;
                self.selected = 0;
                self.apply_filter();
            }
            KeyCode::Char('3') => {
                self.filter = ListFilter::Overdue;
                self.selected = 0;
                self.apply_filter();
            }
            KeyCode::Char('4') => {
                self.filter = ListFilter::Today;
                self.selected = 0;
                self.apply_filter();
            }
            KeyCode::Char('0') => {
                self.filter = ListFilter::All;
                self.selected = 0;
                self.apply_filter();
            }
            _ => {}
        }
    }

    fn handle_add_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => {
                self.screen = Screen::List;
                self.input_buffer.clear();
            }
            KeyCode::Tab => {
                self.editing_field = match self.editing_field {
                    EditField::Title => EditField::Priority,
                    EditField::Priority => EditField::DueDate,
                    EditField::DueDate => EditField::Category,
                    EditField::Category => EditField::Title,
                    _ => EditField::Title,
                };
            }
            KeyCode::Enter => {
                if !self.input_buffer.is_empty() {
                    let _ = self.db.add_task(&self.input_buffer, None, 2, None, None, None);
                    self.screen = Screen::List;
                    self.input_buffer.clear();
                    self.refresh_tasks();
                }
            }
            KeyCode::Backspace => {
                self.input_buffer.pop();
            }
            KeyCode::Char(c) => {
                self.input_buffer.push(c);
            }
            _ => {}
        }
    }

    fn handle_edit_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => {
                self.screen = Screen::List;
                self.input_buffer.clear();
                self.editing_id = None;
            }
            KeyCode::Enter => {
                if let Some(id) = self.editing_id {
                    let _ = self.db.update_task(id, Some(&self.input_buffer), None, None, None, None, None);
                    self.screen = Screen::List;
                    self.input_buffer.clear();
                    self.editing_id = None;
                    self.refresh_tasks();
                }
            }
            KeyCode::Backspace => {
                self.input_buffer.pop();
            }
            KeyCode::Char(c) => {
                self.input_buffer.push(c);
            }
            _ => {}
        }
    }

    fn handle_detail_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.screen = Screen::List;
            }
            KeyCode::Up => {
                if self.detail_scroll > 0 {
                    self.detail_scroll -= 1;
                }
            }
            KeyCode::Down => {
                self.detail_scroll += 1;
            }
            _ => {}
        }
    }

    fn handle_stats_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.screen = Screen::List;
            }
            _ => {}
        }
    }

    fn handle_search_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => {
                self.screen = Screen::List;
                self.input_buffer.clear();
            }
            KeyCode::Enter => {
                if !self.input_buffer.is_empty() {
                    self.filtered_tasks = self.db.search_tasks(&self.input_buffer)
                        .expect("Search failed")
                        .into_iter()
                        .filter(|t| !t.completed)
                        .collect();
                    self.screen = Screen::List;
                    self.selected = 0;
                } else {
                    self.screen = Screen::List;
                }
            }
            KeyCode::Backspace => {
                self.input_buffer.pop();
            }
            KeyCode::Char(c) => {
                self.input_buffer.push(c);
            }
            _ => {}
        }
    }

    fn handle_help_key(&mut self, _key: KeyCode) {}
}

fn ui(f: &mut Frame, app: &App) {
    if app.show_help {
        draw_help(f, app);
        return;
    }

    match app.screen {
        Screen::List => draw_list_screen(f, app),
        Screen::AddTask => draw_add_task_screen(f, app),
        Screen::EditTask => draw_edit_task_screen(f, app),
        Screen::TaskDetail => draw_task_detail_screen(f, app),
        Screen::Stats => draw_stats_screen(f, app),
        Screen::Search => draw_search_screen(f, app),
        Screen::Help => draw_help(f, app),
    }
}

fn draw_list_screen(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([Constraint::Min(3), Constraint::Length(4)])
        .split(f.size());

    draw_list(f, app, chunks[0]);
    draw_list_footer(f, app, chunks[1]);
}

fn draw_list(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .filtered_tasks
        .iter()
        .enumerate()
        .map(|(idx, task)| {
            let status = if task.completed { "✓" } else { "●" };
            let overdue_indicator = if task.is_overdue() { " ⚠" } else { "" };

            let priority_emoji = task.priority_emoji();
            let title = if task.completed {
                task.title.clone().strikethrough().to_string()
            } else {
                task.title.clone()
            };

            let due_str = if let Some(due) = task.format_due_date() {
                format!(" [{}]", due)
            } else {
                String::new()
            };

            let category_str = if let Some(cat) = &task.category {
                format!(" #{}", cat)
            } else {
                String::new()
            };

            let style = if idx == app.selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else if task.completed {
                Style::default().fg(Color::DarkGray).add_modifier(Modifier::DIM)
            } else if task.is_overdue() {
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
            } else if task.is_due_today() {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };

            let text = format!(
                "{:>2} {} {} {}{}{}{}",
                task.id,
                status,
                priority_emoji,
                title,
                due_str,
                category_str,
                overdue_indicator
            );

            ListItem::new(text).style(style)
        })
        .collect();

    let filter_label = match app.filter {
        ListFilter::All => "All Tasks",
        ListFilter::Pending => "Pending",
        ListFilter::Completed => "Completed",
        ListFilter::Overdue => "Overdue",
        ListFilter::Today => "Due Today",
    };

    let title = format!(" {} ({}) ", filter_label, app.filtered_tasks.len());

    let list = List::new(items)
        .block(
            Block::default()
                .title(title)
                .title_alignment(Alignment::Left)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .style(Style::default().fg(Color::White));

    f.render_widget(list, area);
}

fn draw_list_footer(f: &mut Frame, _app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let left_shortcuts = vec![Line::from(vec![
        Span::styled("n", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        Span::raw(" add  "),
        Span::styled("e", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        Span::raw(" edit  "),
        Span::styled("d", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        Span::raw(" delete  "),
        Span::styled("⏎", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        Span::raw(" details"),
    ])];

    let left = Paragraph::new(left_shortcuts)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::TOP));

    f.render_widget(left, chunks[0]);

    let right_shortcuts = vec![Line::from(vec![
        Span::styled("0", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        Span::raw(" all  "),
        Span::styled("3", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        Span::raw(" overdue  "),
        Span::styled("s", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        Span::raw(" stats  "),
        Span::styled("?", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        Span::raw(" help"),
    ])];

    let right = Paragraph::new(right_shortcuts)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Right)
        .block(Block::default().borders(Borders::TOP));

    f.render_widget(right, chunks[1]);
}

fn draw_add_task_screen(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints([Constraint::Min(10)])
        .split(f.size());

    let block = Block::default()
        .title(" ✨ Add New Task ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));

    let inner = block.inner(chunks[0]);
    f.render_widget(block, chunks[0]);

    let inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3); 1])
        .split(inner);

    let title_style = if matches!(app.editing_field, EditField::Title) {
        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let title_input = Paragraph::new(app.input_buffer.clone())
        .block(Block::default().title("Task Title").borders(Borders::ALL))
        .style(title_style);

    f.render_widget(title_input, inner_chunks[0]);
}

fn draw_edit_task_screen(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints([Constraint::Min(10)])
        .split(f.size());

    let block = Block::default()
        .title(" ✏️  Edit Task ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let inner = block.inner(chunks[0]);
    f.render_widget(block, chunks[0]);

    let inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3)])
        .split(inner);

    let title_style = if matches!(app.editing_field, EditField::Title) {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let title_input = Paragraph::new(app.input_buffer.clone())
        .block(Block::default().title("Task Title").borders(Borders::ALL))
        .style(title_style);

    f.render_widget(title_input, inner_chunks[0]);
}

fn draw_task_detail_screen(f: &mut Frame, app: &App) {
    if let Some(task) = app.filtered_tasks.get(app.selected) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Min(10)])
            .split(f.size());

        let border_color = if task.is_overdue() {
            Color::Red
        } else if task.is_due_today() {
            Color::Yellow
        } else {
            Color::Cyan
        };

        let status_emoji = if task.completed { "✅" } else { "📋" };

        let mut content = vec![
            Line::from(vec![
                Span::styled("ID: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(task.id.to_string()),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Title: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(&task.title, Style::default().fg(Color::Cyan)),
            ]),
        ];

        if let Some(desc) = &task.description {
            content.push(Line::from(""));
            content.push(Line::from(vec![
                Span::styled("Description: ", Style::default().add_modifier(Modifier::BOLD)),
            ]));
            for line in desc.lines() {
                content.push(Line::from(vec![Span::raw("  ".to_string() + line)]));
            }
        }

        content.push(Line::from(""));
        content.push(Line::from(vec![
            Span::styled("Priority: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(task.priority_label(), Style::default().fg(Color::Yellow)),
        ]));

        if let Some(due) = task.format_due_date() {
            content.push(Line::from(vec![
                Span::styled("Due Date: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(due, Style::default().fg(border_color)),
            ]));
        }

        if let Some(cat) = &task.category {
            content.push(Line::from(vec![
                Span::styled("Category: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(cat.clone()),
            ]));
        }

        content.push(Line::from(""));
        content.push(Line::from(vec![
            Span::styled("Status: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(if task.completed { "Completed ✓" } else { "Pending" }),
        ]));

        let paragraph = Paragraph::new(content)
            .block(
                Block::default()
                    .title(format!(" {} Task Details ", status_emoji))
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(border_color)),
            )
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, chunks[0]);
    }
}

fn draw_stats_screen(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints([Constraint::Min(15)])
        .split(f.size());

    let (total, completed, pending) = app.db.get_stats().expect("Failed to get statistics");
    let overdue = app.db.get_overdue_tasks().expect("Failed to get overdue").len() as i32;

    let completion_rate = if total > 0 {
        (completed as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    let block = Block::default()
        .title(" 📊 Statistics ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Magenta));

    let inner = block.inner(chunks[0]);
    f.render_widget(block, chunks[0]);

    let inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(3),
        ])
        .split(inner);

    let stats_text = vec![
        Line::from(vec![
            Span::raw("Total Tasks:        "),
            Span::styled(format!("{:>4}", total), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::raw("Completed:          "),
            Span::styled(format!("{:>4}", completed), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::raw("Pending:            "),
            Span::styled(format!("{:>4}", pending), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
    ];

    let stats = Paragraph::new(stats_text).style(Style::default().fg(Color::White));
    f.render_widget(stats, inner_chunks[0]);

    let overdue_text = vec![Line::from(vec![
        Span::raw("Overdue:            "),
        Span::styled(format!("{:>4}", overdue), Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
    ])];
    let overdue_para = Paragraph::new(overdue_text);
    f.render_widget(overdue_para, inner_chunks[2]);

    let gauge = Gauge::default()
        .block(Block::default().title("Completion Rate"))
        .gauge_style(Style::default().fg(Color::Green))
        .ratio(completion_rate / 100.0)
        .label(format!("{:.1}%", completion_rate));

    f.render_widget(gauge, inner_chunks[4]);
}

fn draw_search_screen(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints([Constraint::Length(5)])
        .split(f.size());

    let block = Block::default()
        .title(" 🔍 Search Tasks ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Magenta));

    let inner = block.inner(chunks[0]);
    f.render_widget(block, chunks[0]);

    let search_input = Paragraph::new(app.input_buffer.clone())
        .block(Block::default().title("Search").borders(Borders::ALL))
        .style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD));

    f.render_widget(search_input, inner);
}

fn draw_help(f: &mut Frame, _app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Min(20)])
        .split(f.size());

    let help_text = vec![
        Line::from(""),
        Line::from(vec![Span::styled("📚 HELP & SHORTCUTS", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))]),
        Line::from(""),
        Line::from(vec![Span::styled("Navigation:", Style::default().add_modifier(Modifier::BOLD))]),
        Line::from("  ↑/↓          Navigate tasks"),
        Line::from("  Enter        View task details"),
        Line::from("  Esc          Back to list"),
        Line::from(""),
        Line::from(vec![Span::styled("Task Management:", Style::default().add_modifier(Modifier::BOLD))]),
        Line::from("  n            Add new task"),
        Line::from("  e            Edit task"),
        Line::from("  d            Delete task"),
        Line::from("  Space        Toggle completion"),
        Line::from(""),
        Line::from(vec![Span::styled("Filtering:", Style::default().add_modifier(Modifier::BOLD))]),
        Line::from("  0            Show all tasks"),
        Line::from("  1            Show pending tasks"),
        Line::from("  2            Show completed tasks"),
        Line::from("  3            Show overdue tasks"),
        Line::from("  4            Show due today"),
        Line::from("  /            Search tasks"),
        Line::from(""),
        Line::from(vec![Span::styled("Other:", Style::default().add_modifier(Modifier::BOLD))]),
        Line::from("  s            Show statistics"),
        Line::from("  ?            Show this help"),
        Line::from("  q            Quit"),
        Line::from(""),
        Line::from(vec![Span::styled("Press ? or Esc to close", Style::default().fg(Color::Yellow))]),
    ];

    let paragraph = Paragraph::new(help_text)
        .block(
            Block::default()
                .title(" 📖 Keyboard Shortcuts ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green)),
        )
        .style(Style::default().fg(Color::White));

    f.render_widget(paragraph, chunks[0]);
}

pub fn run(db: &Database) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new(db)?;
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if crossterm::event::poll(std::time::Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                app.handle_key(key.code);
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}