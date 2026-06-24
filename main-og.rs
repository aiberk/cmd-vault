use copypasta::{ClipboardContext, ClipboardProvider};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
    Terminal,
};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(PartialEq)]
enum InputMode {
    Normal,
    Search,
    AddName,
    AddCommand,
    AddDesc,
    ConfirmDelete,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct CommandItem {
    name: String,
    command: String,
    desc: String,
}

struct App {
    items: Vec<CommandItem>,
    state: ListState,
    search_query: String,
    input_mode: InputMode,
    new_name: String,
    new_command: String,
    new_desc: String,
}

impl App {
    fn new() -> App {
        let mut state = ListState::default();
        state.select(Some(0));
        
        let items = Self::load_from_file().unwrap_or_else(|_| vec![
            CommandItem {
                name: "FFmpeg Compress Video".into(),
                command: "ffmpeg -i input.mp4 -vcodec libx265 -crf 28 output.mp4".into(),
                desc: "Compresses mp4 video using the efficient H.265 codec to save space.".into(),
            },
        ]);

        App {
            items,
            state,
            search_query: String::new(),
            input_mode: InputMode::Normal,
            new_name: String::new(),
            new_command: String::new(),
            new_desc: String::new(),
        }
    }

    fn get_file_path() -> PathBuf {
        let mut path = std::env::var("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("."));
        path.push(".cmd-vault.json");
        path
    }

    fn load_from_file() -> Result<Vec<CommandItem>, Box<dyn std::error::Error>> {
        let path = Self::get_file_path();
        if !path.exists() {
            return Err("File does not exist".into());
        }
        let file = File::open(path)?;
        let items = serde_json::from_reader(file)?;
        Ok(items)
    }

    fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::get_file_path();
        let mut file = File::create(path)?;
        let json = serde_json::to_string_pretty(&self.items)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    fn filtered_items(&self) -> Vec<CommandItem> {
        self.items
            .iter()
            .filter(|item| {
                item.name.to_lowercase().contains(&self.search_query.to_lowercase())
                    || item.command.to_lowercase().contains(&self.search_query.to_lowercase())
            })
            .cloned()
            .collect()
    }

    fn remove_current_selection(&mut self) -> Option<String> {
        let filtered = self.filtered_items();
        if let Some(selected_idx) = self.state.selected() {
            if selected_idx < filtered.len() {
                let target_item = &filtered[selected_idx];
                if let Some(master_idx) = self.items.iter().position(|x| x.name == target_item.name && x.command == target_item.command) {
                    let deleted_name = self.items[master_idx].name.clone();
                    self.items.remove(master_idx);
                    let _ = self.save_to_file();
                    
                    let next_len = self.filtered_items().len();
                    if next_len == 0 {
                        self.state.select(None);
                    } else if selected_idx >= next_len {
                        self.state.select(Some(next_len - 1));
                    }
                    return Some(deleted_name);
                }
            }
        }
        None
    }

    pub fn next(&mut self) {
        let filtered_len = self.filtered_items().len();
        if filtered_len == 0 { self.state.select(None); return; }
        let i = match self.state.selected() {
            Some(i) => if i >= filtered_len - 1 { 0 } else { i + 1 },
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let filtered_len = self.filtered_items().len();
        if filtered_len == 0 { self.state.select(None); return; }
        let i = match self.state.selected() {
            Some(i) => if i == 0 { filtered_len - 1 } else { i - 1 },
            None => 0,
        };
        self.state.select(Some(i));
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ].as_ref())
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ].as_ref())
        .split(popup_layout[1])[1]
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let mut clipboard_status = String::from("Ready");

    loop {
        let total_matches_count = app.filtered_items().len();
        if let Some(selected) = app.state.selected() {
            if total_matches_count == 0 { app.state.select(None); }
            else if selected >= total_matches_count { app.state.select(Some(total_matches_count - 1)); }
        } else if total_matches_count > 0 {
            app.state.select(Some(0));
        }

        terminal.draw(|f| {
            let main_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(5), Constraint::Length(3)].as_ref())
                .split(f.area());

            // 1. TOP NAVBAR
            let top_navbar = Paragraph::new(" 📦 CmdVault │ q Quit │ a Add New │ d Delete Selected │ / Search │ y Copy ")
                .block(Block::default().borders(Borders::ALL).style(Style::default().fg(Color::DarkGray)));
            f.render_widget(top_navbar, main_layout[0]);

            // 2. CORE WORKSPACE
            let core_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                .split(main_layout[1]);

            let left_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(2)].as_ref())
                .split(core_layout[0]);

            let search_border_color = if app.input_mode == InputMode::Search { Color::Yellow } else { Color::Cyan };
            let search_bar = Paragraph::new(format!(" {}", app.search_query))
                .block(Block::default().borders(Borders::ALL).title(" Search ").border_style(Style::default().fg(search_border_color)));
            f.render_widget(search_bar, left_layout[0]);

            let displayed_matches = app.filtered_items();
            let list_items: Vec<ListItem> = displayed_matches
                .iter()
                .map(|i| ListItem::new(format!(" {}", i.name)).style(Style::default().fg(Color::White)))
                .collect();

            let list = List::new(list_items)
                .block(Block::default().borders(Borders::ALL).title(" Directory Index "))
                .highlight_style(Style::default().bg(Color::Indexed(24)).fg(Color::White).add_modifier(Modifier::BOLD))
                .highlight_symbol("▶ ");

            f.render_stateful_widget(list, left_layout[1], &mut app.state);

            // Right View Details Panel
            if let Some(selected_index) = app.state.selected() {
                if selected_index < displayed_matches.len() {
                    let current_item = &displayed_matches[selected_index];
                    let detail_text = format!(
                        "\n 💻 COMMAND CONFIGURATION:\n\n   {}\n\n\n 📝 METADATA SPECIFICATION:\n\n   {}",
                        current_item.command, current_item.desc
                    );
                    let details = Paragraph::new(detail_text)
                        .block(Block::default().borders(Borders::ALL).title(" Command Details ").border_style(Style::default().fg(Color::DarkGray)));
                    f.render_widget(details, core_layout[1]);
                }
            } else {
                let empty_notice = Paragraph::new("\n\n   ⚠️ No search results found matches within local vault schema.")
                    .block(Block::default().borders(Borders::ALL).title(" Command Details "));
                f.render_widget(empty_notice, core_layout[1]);
            }

            // 3. FOOTER
            let active_index_display = match app.state.selected() { Some(idx) => idx + 1, None => 0 };
            let status_msg = format!(" │ Status: {} ", clipboard_status);
            let counter_msg = format!(" {}/{} ", active_index_display, total_matches_count);
            
            let bottom_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Min(10), Constraint::Length(10)].as_ref())
                .split(main_layout[2]);

            f.render_widget(Paragraph::new(status_msg).block(Block::default().borders(Borders::ALL).style(Style::default().fg(Color::DarkGray))), bottom_chunks[0]);
            f.render_widget(Paragraph::new(counter_msg).block(Block::default().borders(Borders::ALL).style(Style::default().fg(Color::DarkGray))), bottom_chunks[1]);

            // ADD NEW MODAL DIALOG OVERLAY
            if matches!(app.input_mode, InputMode::AddName | InputMode::AddCommand | InputMode::AddDesc) {
                let popup_area = centered_rect(60, 50, f.area());
                f.render_widget(Clear, popup_area);

                let popup_block = Block::default()
                    .borders(Borders::ALL)
                    .title(" Add New Command Spec ── [TAB] Cycle fields │ [ENTER] Save ")
                    .border_style(Style::default().fg(Color::Magenta));
                f.render_widget(popup_block, popup_area);

                let form_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(3), Constraint::Length(3), Constraint::Min(3)].as_ref())
                    .margin(2)
                    .split(popup_area);

                let f1_color = if app.input_mode == InputMode::AddName { Color::Yellow } else { Color::DarkGray };
                let f2_color = if app.input_mode == InputMode::AddCommand { Color::Yellow } else { Color::DarkGray };
                let f3_color = if app.input_mode == InputMode::AddDesc { Color::Yellow } else { Color::DarkGray };

                f.render_widget(Paragraph::new(format!(" {}", app.new_name)).block(Block::default().borders(Borders::ALL).title(" 1. Shortcut Alias Name ").border_style(Style::default().fg(f1_color))), form_layout[0]);
                f.render_widget(Paragraph::new(format!(" {}", app.new_command)).block(Block::default().borders(Borders::ALL).title(" 2. Exact Execution String ").border_style(Style::default().fg(f2_color))), form_layout[1]);
                f.render_widget(Paragraph::new(format!(" {}", app.new_desc)).block(Block::default().borders(Borders::ALL).title(" 3. Functional Context Breakdown ").border_style(Style::default().fg(f3_color))), form_layout[2]);
            }

            // --- FIXED & BEAUTIFIED SAFETY CONFIRMATION OVERLAY ---
            if app.input_mode == InputMode::ConfirmDelete {
                // Increased size dynamically to 55% width and 30% height to ensure zero clipping
                let confirm_area = centered_rect(55, 30, f.area());
                f.render_widget(Clear, confirm_area);

                let confirm_block = Block::default()
                    .borders(Borders::ALL)
                    .title(" Safety Confirmation ")
                    .border_style(Style::default().fg(Color::LightRed));

                // Clean multi-line layout with structured spacing and text alignments centered
                let confirm_text = "\n⚠️  ARE YOU SURE YOU WANT TO DELETE THIS SELECTION?\n\n\n[y] Confirm Destruction    │    [n / Esc] Abort Actions";
                let confirm_paragraph = Paragraph::new(confirm_text)
                    .block(confirm_block)
                    .alignment(Alignment::Center)
                    .style(Style::default().add_modifier(Modifier::BOLD));
                
                f.render_widget(confirm_paragraph, confirm_area);
            }
        })?;

        // STATE INTERACTION EVENT ROUTER
        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('/') => {
                        app.input_mode = InputMode::Search;
                        clipboard_status = "SEARCH MODE ACTIVE".into();
                    }
                    KeyCode::Char('a') => {
                        app.input_mode = InputMode::AddName;
                        app.new_name.clear(); app.new_command.clear(); app.new_desc.clear();
                        clipboard_status = "FORM MODE: Complete fields architecture setup".into();
                    }
                    KeyCode::Char('d') => {
                        app.input_mode = InputMode::ConfirmDelete;
                        clipboard_status = "⚠️ Awaiting destruction safety clearance...".into();
                    }
                    KeyCode::Down | KeyCode::Char('j') => app.next(),
                    KeyCode::Up | KeyCode::Char('k') => app.previous(),
                    KeyCode::Char('y') => {
                        let displayed = app.filtered_items();
                        if let Some(idx) = app.state.selected() {
                            if idx < displayed.len() {
                                if let Ok(mut ctx) = ClipboardContext::new() {
                                    let _ = ctx.set_contents(displayed[idx].command.clone());
                                    clipboard_status = "✅ YANKED SNIPPET SECURELY TO CLIPBOARD!".into();
                                }
                            }
                        }
                    }
                    _ => {}
                },
                InputMode::Search => match key.code {
                    KeyCode::Esc | KeyCode::Enter => app.input_mode = InputMode::Normal,
                    KeyCode::Backspace => { app.search_query.pop(); },
                    KeyCode::Char(c) => app.search_query.push(c),
                    _ => {}
                },
                InputMode::ConfirmDelete => match key.code {
                    KeyCode::Char('y') | KeyCode::Enter => {
                        if let Some(deleted_name) = app.remove_current_selection() {
                            clipboard_status = format!("🗑️ Pruned target configuration: '{}'", deleted_name);
                        }
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char('n') | KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                        clipboard_status = "Ready (Destruction sequence aborted safely)".into();
                    }
                    _ => {}
                },
                InputMode::AddName | InputMode::AddCommand | InputMode::AddDesc => match key.code {
                    KeyCode::Esc => app.input_mode = InputMode::Normal,
                    KeyCode::Tab => {
                        app.input_mode = match app.input_mode {
                            InputMode::AddName => InputMode::AddCommand,
                            InputMode::AddCommand => InputMode::AddDesc,
                            _ => InputMode::AddName,
                        };
                    }
                    KeyCode::Enter => {
                        if !app.new_name.is_empty() && !app.new_command.is_empty() {
                            app.items.push(CommandItem {
                                name: app.new_name.clone(),
                                command: app.new_command.clone(),
                                desc: app.new_desc.clone(),
                            });
                            let _ = app.save_to_file();
                            app.input_mode = InputMode::Normal;
                            clipboard_status = "💾 Successfully synchronized configuration down to local storage schema!".into();
                        }
                    }
                    KeyCode::Backspace => {
                        match app.input_mode {
                            InputMode::AddName => { app.new_name.pop(); }
                            InputMode::AddCommand => { app.new_command.pop(); }
                            InputMode::AddDesc => { app.new_desc.pop(); }
                            _ => {}
                        }
                    }
                    KeyCode::Char(c) => {
                        match app.input_mode {
                            InputMode::AddName => app.new_name.push(c),
                            InputMode::AddCommand => app.new_command.push(c),
                            InputMode::AddDesc => app.new_desc.push(c),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}