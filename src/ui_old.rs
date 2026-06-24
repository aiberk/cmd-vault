use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
};

use crate::app::{App, InputMode};

/// Renders the entire UI for a single frame.
pub fn draw(f: &mut Frame, app: &mut App) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.area());

    draw_navbar(f, main_layout[0]);
    draw_workspace(f, app, main_layout[1]);
    draw_footer(f, app, main_layout[2]);

    // Overlays
    if matches!(
        app.input_mode,
        InputMode::AddName | InputMode::AddCommand | InputMode::AddDesc
    ) {
        draw_add_modal(f, app);
    }

    if app.input_mode == InputMode::ConfirmDelete {
        draw_confirm_delete(f);
    }

    if app.input_mode == InputMode::FillPlaceholder {
        draw_placeholder_modal(f, app);
    }

    if app.input_mode == InputMode::CopyChoice {
        draw_copy_choice(f);
    }

    if app.input_mode == InputMode::CopiedConfirm {
        draw_copied_confirm(f, app);
    }

    if app.input_mode == InputMode::SortPicker {
        draw_sort_picker(f, app);
    }

    if app.input_mode == InputMode::ExpandedView {
        draw_expanded_view(f, app);
    }
}

fn draw_navbar(f: &mut Frame, area: Rect) {
    let navbar = Paragraph::new(
        " 📦 CmdVault │ ↑↓ Navigate │ ⏎ Expand │ / Search │ y Copy │ a Add │ d Del │ s Sort │ q Quit ",
    )
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::DarkGray)),
    );
    f.render_widget(navbar, area);
}

fn draw_workspace(f: &mut Frame, app: &mut App, area: Rect) {
    let core_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(area);

    draw_sidebar(f, app, core_layout[0]);
    draw_detail_panel(f, app, core_layout[1]);
}

fn draw_sidebar(f: &mut Frame, app: &mut App, area: Rect) {
    let left_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(2)].as_ref())
        .split(area);

    // Search bar
    let search_border_color = if app.input_mode == InputMode::Search {
        Color::Yellow
    } else {
        Color::Cyan
    };
    let search_bar = Paragraph::new(format!(" {}", app.search_query)).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Search ")
            .border_style(Style::default().fg(search_border_color)),
    );
    f.render_widget(search_bar, left_layout[0]);

    // Command list — always shows ALL items regardless of search
    let all_items = app.all_items_sorted();
    let list_items: Vec<ListItem> = all_items
        .iter()
        .map(|i| ListItem::new(format!(" {}", i.name)).style(Style::default().fg(Color::White)))
        .collect();

    let highlight_style = if app.has_search() {
        // Dim the sidebar highlight when search results are active
        Style::default().bg(Color::Indexed(236)).fg(Color::DarkGray)
    } else {
        Style::default()
            .bg(Color::Indexed(24))
            .fg(Color::White)
            .add_modifier(Modifier::BOLD)
    };

    let list = List::new(list_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Directory Index "),
        )
        .highlight_style(highlight_style)
        .highlight_symbol("▶ ");

    f.render_stateful_widget(list, left_layout[1], &mut app.state);
}

fn draw_detail_panel(f: &mut Frame, app: &mut App, area: Rect) {
    if app.has_search() {
        // Show search results in the right panel
        draw_search_results(f, app, area);
    } else {
        // Show command details for the selected sidebar item
        draw_command_details(f, app, area);
    }
}

fn draw_search_results(f: &mut Frame, app: &mut App, area: Rect) {
    let results = app.search_results();
    let title = format!(" Search Results ({} matches) │ ⏎ Expand ", results.len());

    if results.is_empty() {
        let empty = Paragraph::new("\n\n   ⚠️ No matches found for your query.").block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(Style::default().fg(Color::Yellow)),
        );
        f.render_widget(empty, area);
        return;
    }

    // Available width for text (minus borders, highlight symbol, padding)
    let available_width = area.width.saturating_sub(6) as usize;

    let list_items: Vec<ListItem> = results
        .iter()
        .map(|item| {
            let full_text = format!("{} │ {}", item.name, item.command);
            let display_text = if full_text.len() > available_width {
                format!(" {}…", &full_text[..available_width.saturating_sub(2)])
            } else {
                format!(" {}", full_text)
            };
            ListItem::new(display_text).style(Style::default().fg(Color::White))
        })
        .collect();

    let list = List::new(list_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(Style::default().fg(Color::Yellow)),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Indexed(24))
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    f.render_stateful_widget(list, area, &mut app.search_results_state);
}

fn draw_command_details(f: &mut Frame, app: &App, area: Rect) {
    let all_items = app.all_items_sorted();

    if let Some(selected_index) = app.state.selected()
        && selected_index < all_items.len()
    {
        let current_item = &all_items[selected_index];
        let detail_text = format!(
            "\n 💻 COMMAND CONFIGURATION:\n\n   {}\n\n\n 📝 METADATA SPECIFICATION:\n\n   {}",
            current_item.command, current_item.desc
        );
        let details = Paragraph::new(detail_text).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Command Details ")
                .border_style(Style::default().fg(Color::DarkGray)),
        );
        f.render_widget(details, area);
        return;
    }

    let empty_notice = Paragraph::new("\n\n   Select a command to view details.").block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Command Details "),
    );
    f.render_widget(empty_notice, area);
}

fn draw_footer(f: &mut Frame, app: &App, area: Rect) {
    let (active_index_display, total_count) = if app.has_search() {
        let total = app.search_results().len();
        let active = match app.search_results_state.selected() {
            Some(idx) => idx + 1,
            None => 0,
        };
        (active, total)
    } else {
        let total = app.all_items_sorted().len();
        let active = match app.state.selected() {
            Some(idx) => idx + 1,
            None => 0,
        };
        (active, total)
    };

    let status_msg = format!(" │ Status: {} ", app.status_message);
    let counter_msg = format!(" {}/{} ", active_index_display, total_count);

    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(10), Constraint::Length(10)].as_ref())
        .split(area);

    f.render_widget(
        Paragraph::new(status_msg).block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::DarkGray)),
        ),
        bottom_chunks[0],
    );
    f.render_widget(
        Paragraph::new(counter_msg).block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::DarkGray)),
        ),
        bottom_chunks[1],
    );
}

fn draw_add_modal(f: &mut Frame, app: &App) {
    let popup_area = centered_rect(60, 50, f.area());
    f.render_widget(Clear, popup_area);

    let popup_block = Block::default()
        .borders(Borders::ALL)
        .title(" Add New Command ── [ENTER/TAB] Next Field │ [ENTER on Desc] Save │ [ESC] Cancel ")
        .border_style(Style::default().fg(Color::Magenta));
    f.render_widget(popup_block, popup_area);

    let form_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(3),
            ]
            .as_ref(),
        )
        .margin(2)
        .split(popup_area);

    // Live duplicate detection for the name field
    let name_is_duplicate = !app.new_name.is_empty()
        && app
            .items
            .iter()
            .any(|item| item.name.to_lowercase() == app.new_name.to_lowercase());

    let f1_color = if name_is_duplicate {
        Color::Red
    } else if app.input_mode == InputMode::AddName {
        Color::Yellow
    } else {
        Color::DarkGray
    };
    let f2_color = if app.input_mode == InputMode::AddCommand {
        Color::Yellow
    } else {
        Color::DarkGray
    };
    let f3_color = if app.input_mode == InputMode::AddDesc {
        Color::Yellow
    } else {
        Color::DarkGray
    };

    let name_title = if name_is_duplicate {
        " 1. Name (⚠️ DUPLICATE — must be unique) "
    } else {
        " 1. Shortcut Alias Name (must be unique) "
    };

    f.render_widget(
        Paragraph::new(format!(" {}", app.new_name)).block(
            Block::default()
                .borders(Borders::ALL)
                .title(name_title)
                .border_style(Style::default().fg(f1_color)),
        ),
        form_layout[0],
    );
    f.render_widget(
        Paragraph::new(format!(" {}", app.new_command)).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" 2. Exact Execution String ")
                .border_style(Style::default().fg(f2_color)),
        ),
        form_layout[1],
    );
    f.render_widget(
        Paragraph::new(format!(" {}", app.new_desc)).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" 3. Functional Context Breakdown ")
                .border_style(Style::default().fg(f3_color)),
        ),
        form_layout[2],
    );
}

fn draw_confirm_delete(f: &mut Frame) {
    let confirm_area = centered_rect(45, 30, f.area());
    f.render_widget(Clear, confirm_area);

    let confirm_block = Block::default()
        .borders(Borders::ALL)
        .title(" Safety Confirmation ")
        .border_style(Style::default().fg(Color::LightRed));

    let confirm_text = "\n   ⚠️  ARE YOU SURE YOU WANT TO DELETE?\n\n\n   [y] Confirm Destruction\n\n   [n / Esc] Abort";
    let confirm_paragraph = Paragraph::new(confirm_text)
        .block(confirm_block)
        .style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(confirm_paragraph, confirm_area);
}

/// Creates a centered rectangle within the given area.
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

fn draw_placeholder_modal(f: &mut Frame, app: &App) {
    let popup_area = centered_rect(60, 50, f.area());
    f.render_widget(Clear, popup_area);

    if let Some(ref ps) = app.placeholder_state {
        let total = ps.placeholders.len();
        let current = ps.current_index + 1;
        let title = format!(
            " Fill Variables ── [{}/{}] │ [ENTER/TAB] Next │ [ESC] Cancel ",
            current, total
        );

        let popup_block = Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(Style::default().fg(Color::Cyan));
        f.render_widget(popup_block, popup_area);

        let inner_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Min(3),
                ]
                .as_ref(),
            )
            .margin(2)
            .split(popup_area);

        // Show the command template with placeholders highlighted
        let template_display = Paragraph::new(format!(" {}", ps.template)).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Command Template ")
                .border_style(Style::default().fg(Color::DarkGray)),
        );
        f.render_widget(template_display, inner_layout[0]);

        // Current placeholder input field
        let placeholder_name = &ps.placeholders[ps.current_index];
        let input_title = format!(" <{}> ", placeholder_name);
        let input_field = Paragraph::new(format!(" {}", ps.current_input)).block(
            Block::default()
                .borders(Borders::ALL)
                .title(input_title)
                .border_style(Style::default().fg(Color::Yellow)),
        );
        f.render_widget(input_field, inner_layout[1]);

        // Show previously filled values
        let mut filled_lines = String::new();
        for (i, (name, val)) in ps.placeholders.iter().zip(ps.values.iter()).enumerate() {
            filled_lines.push_str(&format!("   {} ✓ <{}> = \"{}\"\n", i + 1, name, val));
        }
        if filled_lines.is_empty() {
            filled_lines = "   No values filled yet...".to_string();
        }

        let history = Paragraph::new(filled_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Filled Values ")
                .border_style(Style::default().fg(Color::DarkGray)),
        );
        f.render_widget(history, inner_layout[2]);
    }
}

/// Reusable numbered-option modal component.
/// `title` — the modal border title
/// `border_color` — color for the modal border
/// `header` — optional header text shown above the options (e.g. an icon + description)
/// `options` — list of option labels (rendered as [1], [2], etc.)
fn draw_option_modal(
    f: &mut Frame,
    title: &str,
    border_color: Color,
    header: Option<&str>,
    options: &[&str],
    percent_x: u16,
    percent_y: u16,
) {
    let popup_area = centered_rect(percent_x, percent_y, f.area());
    f.render_widget(Clear, popup_area);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" {} ", title))
        .border_style(Style::default().fg(border_color));

    let mut text = String::new();
    if let Some(h) = header {
        text.push_str(&format!("\n   {}\n\n", h));
    } else {
        text.push('\n');
    }

    for (i, option) in options.iter().enumerate() {
        text.push_str(&format!("   [{}] {}\n\n", i + 1, option));
    }
    text.push_str("   [Esc] Cancel");

    let paragraph = Paragraph::new(text)
        .block(block)
        .style(Style::default().fg(Color::White));

    f.render_widget(paragraph, popup_area);
}

fn draw_copy_choice(f: &mut Frame) {
    draw_option_modal(
        f,
        "Copy Options",
        Color::Cyan,
        Some("📋  This command contains <variables>"),
        &["Copy raw (keep placeholders)", "Fill variables first"],
        45,
        28,
    );
}

fn draw_copied_confirm(f: &mut Frame, app: &App) {
    let popup_area = centered_rect(50, 25, f.area());
    f.render_widget(Clear, popup_area);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" ✅ Copied to Clipboard ")
        .border_style(Style::default().fg(Color::Green));

    // Truncate long commands for display
    let display_cmd = if app.last_copied.len() > 60 {
        format!("{}…", &app.last_copied[..57])
    } else {
        app.last_copied.clone()
    };

    let text = format!("\n   {}\n\n\n   Press any key to continue...", display_cmd);
    let paragraph = Paragraph::new(text)
        .block(block)
        .style(Style::default().fg(Color::White));

    f.render_widget(paragraph, popup_area);
}

fn draw_sort_picker(f: &mut Frame, app: &App) {
    let current_label = app.sort_mode.label();
    let title = format!("Sort ── Active: {}", current_label);
    draw_option_modal(
        f,
        &title,
        Color::Yellow,
        None,
        &[
            "A → Z (alphabetical)",
            "Z → A (reverse)",
            "Newest first",
            "Oldest first",
            "Shortest command first",
        ],
        40,
        35,
    );
}

fn draw_expanded_view(f: &mut Frame, app: &App) {
    let popup_area = centered_rect(75, 60, f.area());
    f.render_widget(Clear, popup_area);

    if let Some(item) = app.selected_item() {
        let title = format!(" {} ── [Esc] Close │ [y] Copy │ [↑↓] Navigate ", item.name);

        let block = Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(Style::default().fg(Color::Cyan));
        f.render_widget(block, popup_area);

        let inner_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(5), Constraint::Min(3)].as_ref())
            .margin(2)
            .split(popup_area);

        // Command — highlighted with full text wrapping
        let cmd_block = Block::default()
            .borders(Borders::ALL)
            .title(" 💻 Command ")
            .border_style(Style::default().fg(Color::Green));
        let cmd_paragraph = Paragraph::new(format!("\n {}", item.command))
            .block(cmd_block)
            .style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
            .wrap(ratatui::widgets::Wrap { trim: false });
        f.render_widget(cmd_paragraph, inner_layout[0]);

        // Description / metadata
        let desc_text = if item.desc.is_empty() {
            "   No description provided.".to_string()
        } else {
            format!("   {}", item.desc)
        };

        let desc_block = Block::default()
            .borders(Borders::ALL)
            .title(" 📝 Description ")
            .border_style(Style::default().fg(Color::DarkGray));
        let desc_paragraph = Paragraph::new(format!("\n{}", desc_text))
            .block(desc_block)
            .wrap(ratatui::widgets::Wrap { trim: false });
        f.render_widget(desc_paragraph, inner_layout[1]);
    }
}
