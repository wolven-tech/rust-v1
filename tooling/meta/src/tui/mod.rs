use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::io::{self, Write};
use std::time::{Duration, Instant};
use std::fs::File;

use crate::config::Config;
use crate::execution::{LogMessage, LogReceiver};

pub struct App {
    config: Config,
    selected_project: usize,
    running_tasks: Vec<RunningTask>,
    logs: Vec<LogMessage>,
    log_rx: Option<LogReceiver>,
    should_quit: bool,
    filter_project: Option<String>,
    max_logs: usize,
    log_scroll: usize,
    auto_scroll: bool,
    search_buffer: String,
    search_mode: bool,
    export_message: Option<(String, Instant)>,
}

struct RunningTask {
    name: String,
    status: TaskStatus,
    tool: String,
}

#[derive(Clone)]
enum TaskStatus {
    Pending,
    Running,
    Success,
    Failed,
}

impl App {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            selected_project: 0,
            running_tasks: Vec::new(),
            logs: Vec::new(),
            log_rx: None,
            should_quit: false,
            filter_project: None,
            max_logs: 1000,
            log_scroll: 0,
            auto_scroll: true,
            search_buffer: String::new(),
            search_mode: false,
            export_message: None,
        }
    }

    pub fn with_log_receiver(mut self, log_rx: LogReceiver) -> Self {
        self.log_rx = Some(log_rx);
        self
    }

    pub fn run(&mut self) -> Result<()> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Initialize running tasks
        self.init_tasks();

        // Main loop
        let tick_rate = Duration::from_millis(250);
        let mut last_tick = Instant::now();

        loop {
            terminal.draw(|f| self.ui(f))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    // Handle search mode separately
                    if self.search_mode {
                        match key.code {
                            KeyCode::Esc => {
                                self.search_mode = false;
                                self.search_buffer.clear();
                            }
                            KeyCode::Enter => {
                                self.search_mode = false;
                                // Filter is already applied
                            }
                            KeyCode::Backspace => {
                                self.search_buffer.pop();
                                self.apply_search();
                            }
                            KeyCode::Char(c) => {
                                self.search_buffer.push(c);
                                self.apply_search();
                            }
                            _ => {}
                        }
                    } else {
                        // Normal mode
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => {
                                self.should_quit = true;
                            }
                            KeyCode::Down | KeyCode::Char('j') => {
                                self.select_next();
                            }
                            KeyCode::Up | KeyCode::Char('k') => {
                                self.select_previous();
                            }
                            KeyCode::Char('g') if key.modifiers.contains(KeyModifiers::NONE) => {
                                // Jump to top project
                                self.selected_project = 0;
                            }
                            KeyCode::Char('G') | KeyCode::Char('g') if key.modifiers.contains(KeyModifiers::SHIFT) => {
                                // Jump to bottom project
                                self.selected_project = self.running_tasks.len().saturating_sub(1);
                            }
                            KeyCode::Enter | KeyCode::Char(' ') => {
                                // Toggle filter for selected project
                                if let Some(task) = self.running_tasks.get(self.selected_project) {
                                    if self.filter_project.as_ref() == Some(&task.name) {
                                        self.filter_project = None;
                                        self.auto_scroll = true;
                                    } else {
                                        self.filter_project = Some(task.name.clone());
                                        self.log_scroll = 0;
                                        self.auto_scroll = true;
                                    }
                                }
                            }
                            KeyCode::Char('c') if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                                // Clear logs
                                self.logs.clear();
                                self.log_scroll = 0;
                            }
                            KeyCode::Char('a') => {
                                // Show all logs (remove filter)
                                self.filter_project = None;
                                self.auto_scroll = true;
                            }
                            KeyCode::Char('e') => {
                                // Export logs to file
                                self.export_logs_to_file();
                            }
                            KeyCode::Char('x') => {
                                // Export to clipboard (if available)
                                self.export_logs_to_clipboard();
                            }
                            KeyCode::Char('s') => {
                                // Save filtered logs only
                                self.save_filtered_logs();
                            }
                            KeyCode::Char('/') => {
                                // Enter search mode
                                self.search_mode = true;
                                self.search_buffer.clear();
                            }
                            KeyCode::Char('n') => {
                                // Jump to next project with the current filter
                                self.select_next();
                            }
                            KeyCode::Char('p') | KeyCode::Char('N') => {
                                // Jump to previous project
                                self.select_previous();
                            }
                            KeyCode::Char('1'..='9') => {
                                // Quick jump to project by number
                                if let KeyCode::Char(c) = key.code {
                                    if let Some(idx) = c.to_digit(10) {
                                        let idx = (idx as usize).saturating_sub(1);
                                        if idx < self.running_tasks.len() {
                                            self.selected_project = idx;
                                        }
                                    }
                                }
                            }
                            KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                                // Ctrl+U: Scroll up half page
                                self.auto_scroll = false;
                                self.log_scroll = self.log_scroll.saturating_sub(15);
                            }
                            KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                                // Ctrl+D: Scroll down half page
                                let filtered_count = self.get_filtered_logs().len();
                                self.log_scroll = self.log_scroll.saturating_add(15)
                                    .min(filtered_count.saturating_sub(1));
                                // Re-enable auto-scroll if at bottom
                                if self.log_scroll >= filtered_count.saturating_sub(1) {
                                    self.auto_scroll = true;
                                }
                            }
                            KeyCode::Char('b') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                                // Ctrl+B: Scroll up full page
                                self.auto_scroll = false;
                                self.log_scroll = self.log_scroll.saturating_sub(30);
                            }
                            KeyCode::Char('f') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                                // Ctrl+F: Scroll down full page
                                let filtered_count = self.get_filtered_logs().len();
                                self.log_scroll = self.log_scroll.saturating_add(30)
                                    .min(filtered_count.saturating_sub(1));
                                // Re-enable auto-scroll if at bottom
                                if self.log_scroll >= filtered_count.saturating_sub(1) {
                                    self.auto_scroll = true;
                                }
                            }
                            KeyCode::PageUp => {
                                // Scroll up
                                self.auto_scroll = false;
                                self.log_scroll = self.log_scroll.saturating_sub(10);
                            }
                            KeyCode::PageDown => {
                                // Scroll down
                                let filtered_count = self.get_filtered_logs().len();
                                self.log_scroll = self.log_scroll.saturating_add(10)
                                    .min(filtered_count.saturating_sub(1));
                                // Re-enable auto-scroll if at bottom
                                if self.log_scroll >= filtered_count.saturating_sub(1) {
                                    self.auto_scroll = true;
                                }
                            }
                            KeyCode::Home => {
                                // Jump to top
                                self.auto_scroll = false;
                                self.log_scroll = 0;
                            }
                            KeyCode::End => {
                                // Jump to bottom and re-enable auto-scroll
                                self.auto_scroll = true;
                                let filtered_count = self.get_filtered_logs().len();
                                self.log_scroll = filtered_count.saturating_sub(1);
                            }
                            _ => {}
                        }
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                // Clear export message after 3 seconds
                if let Some((_, timestamp)) = self.export_message {
                    if timestamp.elapsed() >= Duration::from_secs(3) {
                        self.export_message = None;
                    }
                }

                // Receive new log messages - ALWAYS add to buffer, filtering is done at display time
                if let Some(ref mut log_rx) = self.log_rx {
                    while let Ok(log_msg) = log_rx.try_recv() {
                        self.logs.push(log_msg);

                        // Limit log buffer size
                        if self.logs.len() > self.max_logs {
                            self.logs.remove(0);
                            // Adjust scroll position
                            self.log_scroll = self.log_scroll.saturating_sub(1);
                        }

                        // Auto-scroll to bottom if enabled
                        if self.auto_scroll {
                            let filtered_count = if let Some(ref filter) = self.filter_project {
                                self.logs.iter().filter(|log| &log.project == filter).count()
                            } else {
                                self.logs.len()
                            };
                            self.log_scroll = filtered_count.saturating_sub(1);
                        }
                    }
                }

                // Update task statuses
                self.update_task_status();

                last_tick = Instant::now();
            }

            if self.should_quit {
                break;
            }
        }

        // Restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        Ok(())
    }

    fn init_tasks(&mut self) {
        // Add all configured projects as tasks
        for (name, project) in &self.config.projects {
            if project.tasks.contains_key("dev") {
                self.running_tasks.push(RunningTask {
                    name: name.clone(),
                    status: TaskStatus::Pending,
                    tool: project.tasks.get("dev").unwrap().tool.clone(),
                });
            }
        }
    }

    fn select_next(&mut self) {
        if self.selected_project < self.running_tasks.len().saturating_sub(1) {
            self.selected_project += 1;
        }
    }

    fn select_previous(&mut self) {
        self.selected_project = self.selected_project.saturating_sub(1);
    }

    fn update_task_status(&mut self) {
        // Update task statuses based on log activity
        if !self.logs.is_empty() {
            for task in &mut self.running_tasks {
                // Get recent logs for this project
                let project_logs: Vec<_> = self.logs
                    .iter()
                    .filter(|log| log.project == task.name)
                    .collect();

                if project_logs.is_empty() {
                    continue;
                }

                // Check for errors in recent logs
                let has_errors = project_logs.iter().any(|log| {
                    matches!(log.level, crate::execution::LogLevel::Error)
                });

                // Check for success indicators
                let has_success = project_logs.iter().any(|log| {
                    let lower = log.message.to_lowercase();
                    lower.contains("compiled successfully")
                        || lower.contains("ready")
                        || lower.contains("listening")
                        || lower.contains("started")
                });

                // Update status based on logs
                task.status = if has_errors {
                    TaskStatus::Failed
                } else if has_success {
                    TaskStatus::Success
                } else {
                    TaskStatus::Running
                };
            }
        }
    }

    fn get_filtered_logs(&self) -> Vec<&LogMessage> {
        if let Some(ref filter) = self.filter_project {
            self.logs
                .iter()
                .filter(|log| &log.project == filter)
                .collect()
        } else {
            self.logs.iter().collect()
        }
    }

    fn apply_search(&mut self) {
        if self.search_buffer.is_empty() {
            return;
        }

        let search_lower = self.search_buffer.to_lowercase();

        // Find first matching project
        for (idx, task) in self.running_tasks.iter().enumerate() {
            if task.name.to_lowercase().contains(&search_lower) {
                self.selected_project = idx;
                return;
            }
        }
    }

    fn export_logs_to_file(&mut self) {
        let filtered_logs = self.get_filtered_logs();
        let log_count = filtered_logs.len();

        // Generate filename with timestamp
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let filename = if let Some(ref filter) = self.filter_project {
            format!("meta-logs-{}-{}.txt", filter, timestamp)
        } else {
            format!("meta-logs-all-{}.txt", timestamp)
        };

        match File::create(&filename) {
            Ok(mut file) => {
                for log in &filtered_logs {
                    let level_str = match log.level {
                        crate::execution::LogLevel::Info => "INFO",
                        crate::execution::LogLevel::Error => "ERROR",
                        crate::execution::LogLevel::Debug => "DEBUG",
                    };

                    if let Err(e) = writeln!(
                        file,
                        "[{}] [{}] [{}] {}",
                        log.timestamp, log.project, level_str, log.message
                    ) {
                        self.export_message = Some((
                            format!("❌ Error writing logs: {}", e),
                            Instant::now()
                        ));
                        return;
                    }
                }

                self.export_message = Some((
                    format!("✅ Exported {} logs to {}", log_count, filename),
                    Instant::now()
                ));
            }
            Err(e) => {
                self.export_message = Some((
                    format!("❌ Failed to create file: {}", e),
                    Instant::now()
                ));
            }
        }
    }

    fn export_logs_to_clipboard(&mut self) {
        let filtered_logs = self.get_filtered_logs();
        let log_count = filtered_logs.len();

        let mut output = String::new();
        for log in &filtered_logs {
            let level_str = match log.level {
                crate::execution::LogLevel::Info => "INFO",
                crate::execution::LogLevel::Error => "ERROR",
                crate::execution::LogLevel::Debug => "DEBUG",
            };

            output.push_str(&format!(
                "[{}] [{}] [{}] {}\n",
                log.timestamp, log.project, level_str, log.message
            ));
        }

        // Try to use pbcopy on macOS
        #[cfg(target_os = "macos")]
        {
            use std::process::{Command, Stdio};

            match Command::new("pbcopy")
                .stdin(Stdio::piped())
                .spawn()
            {
                Ok(mut child) => {
                    if let Some(mut stdin) = child.stdin.take() {
                        if let Err(e) = stdin.write_all(output.as_bytes()) {
                            self.export_message = Some((
                                format!("❌ Failed to write to clipboard: {}", e),
                                Instant::now()
                            ));
                            return;
                        }
                        drop(stdin);

                        match child.wait() {
                            Ok(_) => {
                                self.export_message = Some((
                                    format!("✅ Copied {} logs to clipboard", log_count),
                                    Instant::now()
                                ));
                            }
                            Err(e) => {
                                self.export_message = Some((
                                    format!("❌ Clipboard error: {}", e),
                                    Instant::now()
                                ));
                            }
                        }
                    }
                }
                Err(e) => {
                    self.export_message = Some((
                        format!("❌ pbcopy not available: {}. Use 'e' to export to file.", e),
                        Instant::now()
                    ));
                }
            }
        }

        #[cfg(not(target_os = "macos"))]
        {
            self.export_message = Some((
                "❌ Clipboard not available on this platform. Use 'e' to export to file.".to_string(),
                Instant::now()
            ));
        }
    }

    fn save_filtered_logs(&mut self) {
        if self.filter_project.is_none() {
            self.export_message = Some((
                "❌ No filter active. Use 'e' to export all logs.".to_string(),
                Instant::now()
            ));
            return;
        }

        self.export_logs_to_file();
    }

    fn ui(&mut self, f: &mut ratatui::Frame) {
        // Main layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),   // Header
                Constraint::Min(0),      // Main content
                Constraint::Length(3),   // Footer
            ])
            .split(f.area());

        // Header - show export message if present
        let header_content = if let Some((ref message, _)) = self.export_message {
            vec![Line::from(vec![
                Span::styled("Meta ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::raw("Task Orchestrator | "),
                Span::raw(message),
            ])]
        } else {
            vec![Line::from(vec![
                Span::styled("Meta ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::raw("Task Orchestrator"),
            ])]
        };

        let header = Paragraph::new(header_content)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(header, chunks[0]);

        // Main content - split into projects and logs
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(40),  // Projects list
                Constraint::Percentage(60),  // Logs
            ])
            .split(chunks[1]);

        // Projects list
        let projects: Vec<ListItem> = self
            .running_tasks
            .iter()
            .enumerate()
            .map(|(i, task)| {
                let icon = match task.status {
                    TaskStatus::Pending => "⏸  ",
                    TaskStatus::Running => "▶  ",
                    TaskStatus::Success => "✅ ",
                    TaskStatus::Failed => "❌ ",
                };

                let is_selected = i == self.selected_project;
                let is_filtered = self.filter_project.as_ref() == Some(&task.name);

                // Project number for quick jump
                let number = format!("{}. ", i + 1);
                let number_span = Span::styled(number, Style::default().fg(Color::DarkGray));

                let mut spans = vec![
                    number_span,
                    Span::raw(icon),
                ];

                // Highlight filtered project with special marker
                if is_filtered {
                    spans.push(Span::styled("🔍 ", Style::default().fg(Color::Cyan)));
                }

                // Style project name based on selection and filter
                let name_style = if is_selected && is_filtered {
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
                } else if is_selected {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else if is_filtered {
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };

                spans.push(Span::styled(&task.name, name_style));
                spans.push(Span::raw(" "));
                spans.push(Span::styled(
                    format!("({})", &task.tool),
                    Style::default().fg(Color::DarkGray),
                ));

                ListItem::new(Line::from(spans))
            })
            .collect();

        let project_title = if self.search_mode {
            format!("Projects [search: {}]", self.search_buffer)
        } else {
            "Projects [1-9: jump, /: search]".to_string()
        };

        let projects_list = List::new(projects)
            .block(Block::default().title(project_title).borders(Borders::ALL))
            .highlight_symbol("");

        f.render_widget(projects_list, main_chunks[0]);

        // Logs panel - use filtered logs for display
        let filtered_logs = self.get_filtered_logs();

        // Calculate visible window
        let visible_height = main_chunks[1].height.saturating_sub(2) as usize; // subtract borders
        let total_logs = filtered_logs.len();

        // Ensure scroll position is valid
        let scroll_pos = self.log_scroll.min(total_logs.saturating_sub(1));

        // Calculate which logs to show
        let start_idx = if total_logs <= visible_height {
            0
        } else if scroll_pos + visible_height > total_logs {
            total_logs.saturating_sub(visible_height)
        } else {
            scroll_pos
        };

        let end_idx = (start_idx + visible_height).min(total_logs);

        let logs: Vec<ListItem> = filtered_logs
            .iter()
            .skip(start_idx)
            .take(end_idx - start_idx)
            .map(|log| {
                let message_color = match log.level {
                    crate::execution::LogLevel::Info => Color::White,
                    crate::execution::LogLevel::Error => Color::Red,
                    crate::execution::LogLevel::Debug => Color::DarkGray,
                };

                ListItem::new(Line::from(vec![
                    Span::styled(&log.timestamp, Style::default().fg(Color::DarkGray)),
                    Span::raw(" "),
                    Span::styled(&log.project, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                    Span::raw(" │ "),
                    Span::styled(&log.message, Style::default().fg(message_color)),
                ]))
            })
            .collect();

        let scroll_indicator = if self.auto_scroll {
            "⬇ auto"
        } else {
            "⏸ manual"
        };

        let scroll_position = if total_logs > 0 {
            format!("{}/{}", start_idx + 1, total_logs)
        } else {
            "0/0".to_string()
        };

        let log_title = if let Some(ref filter) = self.filter_project {
            format!(
                "Logs: {} [{}] {} - Ctrl+U/D: scroll, Home/End: jump, Enter: toggle",
                filter,
                scroll_indicator,
                scroll_position
            )
        } else {
            format!(
                "Logs: All ({}) [{}] {} - Ctrl+U/D: scroll, PgUp/PgDn: page, Home/End: jump",
                total_logs,
                scroll_indicator,
                scroll_position
            )
        };

        let logs_list = List::new(logs).block(Block::default().title(log_title).borders(Borders::ALL));

        f.render_widget(logs_list, main_chunks[1]);

        // Footer - update based on search mode
        let footer_spans = if self.search_mode {
            vec![
                Span::styled("Search mode: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::raw("type to search | "),
                Span::styled("Enter", Style::default().fg(Color::Yellow)),
                Span::raw(": select | "),
                Span::styled("Esc", Style::default().fg(Color::Yellow)),
                Span::raw(": cancel"),
            ]
        } else {
            vec![
                Span::styled("q", Style::default().fg(Color::Yellow)),
                Span::raw(": quit | "),
                Span::styled("↑↓/jk", Style::default().fg(Color::Yellow)),
                Span::raw(": nav | "),
                Span::styled("Ctrl+U/D", Style::default().fg(Color::Yellow)),
                Span::raw(": scroll | "),
                Span::styled("Enter", Style::default().fg(Color::Yellow)),
                Span::raw(": filter | "),
                Span::styled("e", Style::default().fg(Color::Yellow)),
                Span::raw(": export | "),
                Span::styled("x", Style::default().fg(Color::Yellow)),
                Span::raw(": copy | "),
                Span::styled("/", Style::default().fg(Color::Yellow)),
                Span::raw(": search | "),
                Span::styled("1-9", Style::default().fg(Color::Yellow)),
                Span::raw(": jump"),
            ]
        };

        let footer = Paragraph::new(Line::from(footer_spans))
            .block(Block::default().borders(Borders::ALL));

        f.render_widget(footer, chunks[2]);
    }
}

pub async fn run_tui_with_streaming(config: Config, log_rx: LogReceiver) -> Result<()> {
    let mut app = App::new(config).with_log_receiver(log_rx);
    app.run()
}
