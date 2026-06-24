/// Layout components - navbar, workspace, footer
/// 
/// These are the main structural elements of the UI.
/// Contributors working on layout and navigation work here.

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::{App, InputMode};

/// Draws the top navigation bar with keyboard shortcuts
pub fn draw_navbar(f: &mut Frame, area: Rect) {
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

/// Draws the main workspace area (sidebar + details/search results)
pub fn draw_workspace(f: &mut Frame, app: &mut App, area: Rect) {
    let core_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(area);

    draw_sidebar(f, app, core_layout[0]);
    draw_detail_panel(f, app, core_layout[1]);
}

/// Draws the left sidebar with search bar and command list
pub fn draw_sidebar(f: &mut Frame, app: &mut App, area: Rect) {
    let left_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(2)])
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

/// Draws the right panel (context-dependent content)
pub fn draw_detail_panel(f: &mut Frame, app: &mut App, area: Rect) {
    if app.has_search() {
        // Show search results in the right panel
        draw_search_results(f, app, area);
    } else {
        // Show command details for the selected sidebar item
        draw_details_panel(f, app, area);
    }
}

/// Draws the search results panel (right side when searching)
pub fn draw_search_results(f: &mut Frame, app: &mut App, area: Rect) {
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

/// Draws the command details panel (right side when not searching)
pub fn draw_details_panel(f: &mut Frame, app: &mut App, area: Rect) {
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

    let empty_notice = Paragraph::new("\n\n   Select a command to view details.")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Command Details ")
                .border_style(Style::default().fg(Color::DarkGray)),
        );
    f.render_widget(empty_notice, area);
}

/// Draws the bottom footer with status information
pub fn draw_footer(f: &mut Frame, app: &App, area: Rect) {
    let footer_content = if app.has_search() {
        format!("🔍 Search: '{}' │ {}", app.search_query, app.status_message)
    } else {
        app.status_message.clone()
    };

    let footer = Paragraph::new(footer_content)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray)))
        .style(Style::default().fg(Color::Gray));

    f.render_widget(footer, area);
}