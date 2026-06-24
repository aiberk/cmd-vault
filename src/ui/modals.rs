/// Modal dialogs and overlays
/// 
/// All modal windows are defined here. This makes it easy for contributors
/// to add new modals or modify existing ones without hunting through a massive file.

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

use crate::app::{App, InputMode};
use crate::ui::components::*;

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
    let popup_area = centered_modal_area(f.area(), percent_x, percent_y);
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

/// Modal for adding new commands (3-step form)
pub fn draw_add_modal(f: &mut Frame, app: &App) {
    let area = centered_modal_area(f.area(), 60, 40);
    f.render_widget(Clear, area);

    let current_field = match app.input_mode {
        InputMode::AddName => "Name",
        InputMode::AddCommand => "Command", 
        InputMode::AddDesc => "Description",
        _ => "Name",
    };

    let lines = vec![
        Line::from(""),
        Line::from(vec![Span::raw("  Fill in the command details:")]),
        Line::from(""),
        create_input_field("Name:", &app.new_name, app.input_mode == InputMode::AddName),
        Line::from(""),
        create_input_field("Command:", &app.new_command, app.input_mode == InputMode::AddCommand),
        Line::from(""),
        create_input_field("Description:", &app.new_desc, app.input_mode == InputMode::AddDesc),
        Line::from(""),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("[Tab]", Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
            Span::raw(" Next  "),
            Span::styled("[Enter]", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(" Save  "),
            Span::styled("[Esc]", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw(" Cancel"),
        ]),
        Line::from(""),
    ];

    let title = format!(" Add Command - {} ", current_field);
    let modal = Paragraph::new(lines)
        .block(modal_block(&title))
        .wrap(Wrap { trim: true });

    f.render_widget(modal, area);
}

/// Confirmation dialog for deleting commands
pub fn draw_confirm_delete(f: &mut Frame) {
    let confirm_area = centered_modal_area(f.area(), 50, 30);
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

/// Modal for filling in placeholder variables
pub fn draw_placeholder_modal(f: &mut Frame, app: &App) {
    let popup_area = centered_modal_area(f.area(), 60, 50);
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
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(3),
            ])
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

/// Choice modal for copy options (raw vs filled)
pub fn draw_copy_choice(f: &mut Frame) {
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

/// Confirmation modal after successful copy
pub fn draw_copied_confirm(f: &mut Frame, app: &App) {
    let popup_area = centered_modal_area(f.area(), 50, 25);
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

/// Sort options picker modal
pub fn draw_sort_picker(f: &mut Frame) {
    draw_option_modal(
        f,
        "Sort Commands",
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

/// Expanded view modal for detailed command inspection
pub fn draw_expanded_view(f: &mut Frame, app: &App) {
    let popup_area = centered_modal_area(f.area(), 75, 60);
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
            .constraints([Constraint::Length(5), Constraint::Min(3)])
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
            .wrap(Wrap { trim: false });
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
            .border_style(Style::default().fg(Color::Yellow));
        let desc_paragraph = Paragraph::new(desc_text)
            .block(desc_block)
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: false });
        f.render_widget(desc_paragraph, inner_layout[1]);
    }
}