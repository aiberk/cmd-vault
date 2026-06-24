use crate::app::{App, InputMode};
use crate::ui::{layout::*, modals::*};
/// Main rendering coordinator - orchestrates all UI components
///
/// This is the entry point for UI rendering. It coordinates:
/// - Main layout (navbar, content, footer)  
/// - Modal overlays
/// - State-dependent UI elements
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};

/// Renders the entire UI for a single frame.
/// This is the main entry point called by the application loop.
pub fn draw(f: &mut Frame, app: &mut App) {
    // Create main application layout
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Navbar
            Constraint::Min(5),    // Main content area
            Constraint::Length(3), // Footer
        ])
        .split(f.area());

    // Render main UI components
    draw_navbar(f, main_layout[0]);
    draw_workspace(f, app, main_layout[1]);
    draw_footer(f, app, main_layout[2]);

    // Render modal overlays based on current mode
    match app.input_mode {
        InputMode::AddName | InputMode::AddCommand | InputMode::AddDesc => {
            draw_add_modal(f, app);
        }
        InputMode::ConfirmDelete => {
            draw_confirm_delete(f);
        }
        InputMode::FillPlaceholder => {
            draw_placeholder_modal(f, app);
        }
        InputMode::CopyChoice => {
            draw_copy_choice(f);
        }
        InputMode::CopiedConfirm => {
            draw_copied_confirm(f, app);
        }
        InputMode::SortPicker => {
            draw_sort_picker(f);
        }
        InputMode::ExpandedView => {
            draw_expanded_view(f, app);
        }
        _ => {} // No overlay needed
    }
}
