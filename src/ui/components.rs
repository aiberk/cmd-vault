/// Reusable UI components and utilities
/// 
/// This file contains small, reusable components that can be used
/// across different parts of the UI. Contributors add new widgets here.

use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

/// Creates a centered modal overlay area
pub fn centered_modal_area(area: Rect, width_percent: u16, height_percent: u16) -> Rect {
    let popup_layout = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            ratatui::layout::Constraint::Percentage((100 - height_percent) / 2),
            ratatui::layout::Constraint::Percentage(height_percent),
            ratatui::layout::Constraint::Percentage((100 - height_percent) / 2),
        ])
        .split(area);

    ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            ratatui::layout::Constraint::Percentage((100 - width_percent) / 2),
            ratatui::layout::Constraint::Percentage(width_percent),
            ratatui::layout::Constraint::Percentage((100 - width_percent) / 2),
        ])
        .split(popup_layout[1])[1]
}

/// Creates a standard modal block with consistent styling
pub fn modal_block(title: &str) -> Block<'_> {
    Block::default()
        .borders(Borders::ALL)
        .title(title)
        .border_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .style(Style::default().bg(Color::Black))
}

/// Creates a reusable option modal for numbered choices
#[allow(dead_code)]
pub fn create_option_modal<'a>(title: &'a str, options: &'a [&'a str]) -> Paragraph<'a> {
    let mut lines = vec![Line::from("")]; // Empty line at top

    for (i, option) in options.iter().enumerate() {
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(
                format!("[{}]", i + 1),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
            Span::raw(*option),
        ]));
    }

    lines.push(Line::from("")); // Empty line at bottom

    Paragraph::new(lines)
        .block(modal_block(title))
        .wrap(Wrap { trim: true })
}

/// Creates a confirmation dialog (Yes/No)
#[allow(dead_code)]
pub fn create_confirmation_modal<'a>(title: &'a str, message: &'a str) -> Paragraph<'a> {
    let lines = vec![
        Line::from(""),
        Line::from(vec![Span::raw(format!("  {}", message))]),
        Line::from(""),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("[y]", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(" Yes  "),
            Span::styled("[n]", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw(" No"),
        ]),
        Line::from(""),
    ];

    Paragraph::new(lines)
        .block(modal_block(title))
        .wrap(Wrap { trim: true })
}

/// Creates an info modal for displaying information
#[allow(dead_code)]
pub fn create_info_modal<'a>(title: &'a str, message: &'a str) -> Paragraph<'a> {
    let lines = vec![
        Line::from(""),
        Line::from(vec![Span::raw(format!("  {}", message))]),
        Line::from(""),
        Line::from(vec![
            Span::raw("  Press any key to continue..."),
        ]),
        Line::from(""),
    ];

    Paragraph::new(lines)
        .block(modal_block(title))
        .wrap(Wrap { trim: true })
}

/// Creates a text input field with cursor
pub fn create_input_field<'a>(label: &'a str, value: &'a str, is_focused: bool) -> Line<'a> {
    let cursor = if is_focused { "▎" } else { " " };
    Line::from(vec![
        Span::raw(format!("{}  ", label)),
        Span::styled(
            format!("{}{}", value, cursor),
            if is_focused {
                Style::default().bg(Color::DarkGray)
            } else {
                Style::default()
            },
        ),
    ])
}