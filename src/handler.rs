use copypasta::{ClipboardContext, ClipboardProvider};
use crossterm::event::{self, Event, KeyCode};

use crate::app::{App, InputMode, PlaceholderState};
use crate::constants::{DEFAULT_STATUS, FORM_MODE_STATUS, SEARCH_MODE_STATUS};
use crate::placeholders;
use crate::utils::SortMode;

/// Possible outcomes from processing a key event.
pub enum HandleResult {
    Continue,
    Quit,
}

/// Reads the next terminal event and dispatches it based on the current input mode.
pub fn handle_events(app: &mut App) -> Result<HandleResult, Box<dyn std::error::Error>> {
    if let Event::Key(key) = event::read()? {
        match app.input_mode {
            InputMode::Normal => return handle_normal_mode(app, key.code),
            InputMode::Search => handle_search_mode(app, key.code),
            InputMode::ConfirmDelete => handle_confirm_delete_mode(app, key.code),
            InputMode::FillPlaceholder => handle_placeholder_mode(app, key.code),
            InputMode::CopyChoice => handle_copy_choice_mode(app, key.code),
            InputMode::CopiedConfirm => handle_copied_confirm(app),
            InputMode::SortPicker => handle_sort_picker(app, key.code),
            InputMode::ExpandedView => handle_expanded_view(app, key.code),
            InputMode::AddName | InputMode::AddCommand | InputMode::AddDesc => {
                handle_form_mode(app, key.code)
            }
        }
    }
    Ok(HandleResult::Continue)
}

fn handle_normal_mode(
    app: &mut App,
    code: KeyCode,
) -> Result<HandleResult, Box<dyn std::error::Error>> {
    match code {
        KeyCode::Char('q') | KeyCode::Esc => return Ok(HandleResult::Quit),
        KeyCode::Char('/') => {
            app.input_mode = InputMode::Search;
            app.status_message = SEARCH_MODE_STATUS.into();
        }
        KeyCode::Char('a') => {
            app.input_mode = InputMode::AddName;
            app.new_name.clear();
            app.new_command.clear();
            app.new_desc.clear();
            app.status_message = FORM_MODE_STATUS.into();
        }
        KeyCode::Char('d') => {
            app.input_mode = InputMode::ConfirmDelete;
            app.status_message = "⚠️ Awaiting destruction safety clearance...".into();
        }
        KeyCode::Down | KeyCode::Char('j') => app.next(),
        KeyCode::Up | KeyCode::Char('k') => app.previous(),
        KeyCode::Enter => {
            // Expand selected item detail view
            if app.selected_item().is_some() {
                app.input_mode = InputMode::ExpandedView;
                app.status_message = "Expanded view │ [Esc] Close │ [y] Copy".into();
            }
        }
        KeyCode::Char('y') => initiate_copy(app),
        KeyCode::Char('s') => {
            app.input_mode = InputMode::SortPicker;
            app.status_message = "Sort by...".into();
        }
        _ => {}
    }
    Ok(HandleResult::Continue)
}

fn handle_search_mode(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Esc => app.input_mode = InputMode::Normal,
        KeyCode::Enter => {
            // If there's a selected search result, expand it. Otherwise just exit search.
            if app.has_search() && app.selected_item().is_some() {
                app.input_mode = InputMode::ExpandedView;
                app.status_message = "Expanded view │ [Esc] Close │ [y] Copy".into();
            } else {
                app.input_mode = InputMode::Normal;
            }
        }
        KeyCode::Backspace => {
            app.search_query.pop();
        }
        // Arrow keys navigate search results while still in search mode
        KeyCode::Down => app.next(),
        KeyCode::Up => app.previous(),
        KeyCode::Char(c) => app.search_query.push(c),
        _ => {}
    }
}

fn handle_confirm_delete_mode(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Char('y') | KeyCode::Enter => {
            if let Some(deleted_name) = app.remove_current_selection() {
                app.status_message = format!("🗑️ Pruned target configuration: '{}'", deleted_name);
            }
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Char('n') | KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
            app.status_message = "Ready (Destruction sequence aborted safely)".into();
        }
        _ => {}
    }
}

fn handle_form_mode(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Esc => app.input_mode = InputMode::Normal,
        KeyCode::Tab | KeyCode::Enter => match app.input_mode {
            InputMode::AddName => app.input_mode = InputMode::AddCommand,
            InputMode::AddCommand => app.input_mode = InputMode::AddDesc,
            InputMode::AddDesc => {
                if code == KeyCode::Enter {
                    app.submit_new_command();
                } else {
                    app.input_mode = InputMode::AddName;
                }
            }
            _ => {}
        },
        KeyCode::Backspace => match app.input_mode {
            InputMode::AddName => {
                app.new_name.pop();
            }
            InputMode::AddCommand => {
                app.new_command.pop();
            }
            InputMode::AddDesc => {
                app.new_desc.pop();
            }
            _ => {}
        },
        KeyCode::Char(c) => match app.input_mode {
            InputMode::AddName => app.new_name.push(c),
            InputMode::AddCommand => app.new_command.push(c),
            InputMode::AddDesc => app.new_desc.push(c),
            _ => {}
        },
        _ => {}
    }
}

/// Handles the sort picker dialog.
fn handle_sort_picker(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Char('1') => app.apply_sort(SortMode::AZ),
        KeyCode::Char('2') => app.apply_sort(SortMode::ZA),
        KeyCode::Char('3') => app.apply_sort(SortMode::NewestFirst),
        KeyCode::Char('4') => app.apply_sort(SortMode::OldestFirst),
        KeyCode::Char('5') => app.apply_sort(SortMode::ShortestFirst),
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
            app.status_message = "Sort cancelled".into();
        }
        _ => {}
    }
}

/// Handles the expanded detail view — navigate, copy, or dismiss.
fn handle_expanded_view(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.input_mode = InputMode::Normal;
            app.status_message = DEFAULT_STATUS.into();
        }
        KeyCode::Char('y') => initiate_copy(app),
        KeyCode::Down | KeyCode::Char('j') => {
            app.next();
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.previous();
        }
        _ => {}
    }
}

/// Handles the copy choice dialog: [1] copy raw, [2] fill placeholders.
fn handle_copy_choice_mode(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Char('1') => {
            if let Some(item) = app.selected_item() {
                copy_to_clipboard(app, &item.command);
            }
        }
        KeyCode::Char('2') => {
            if let Some(item) = app.selected_item() {
                let found_placeholders = placeholders::extract_placeholders(&item.command);
                if !found_placeholders.is_empty() {
                    let first_name = found_placeholders[0].clone();
                    let total = found_placeholders.len();
                    app.placeholder_state = Some(PlaceholderState {
                        template: item.command,
                        placeholders: found_placeholders,
                        values: Vec::new(),
                        current_index: 0,
                        current_input: String::new(),
                    });
                    app.input_mode = InputMode::FillPlaceholder;
                    app.status_message = format!("Fill <{}> (1/{})", first_name, total);
                } else {
                    // No placeholders found, copy directly
                    copy_to_clipboard(app, &item.command);
                }
            }
        }
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
            app.status_message = "Copy cancelled".into();
        }
        _ => {}
    }
}

/// Handles the "Copied!" confirmation dialog — any key dismisses it.
fn handle_copied_confirm(app: &mut App) {
    app.input_mode = InputMode::Normal;
    app.status_message = DEFAULT_STATUS.into();
}

/// Handles key events while in the placeholder fill-in modal.
fn handle_placeholder_mode(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Esc => {
            app.placeholder_state = None;
            app.input_mode = InputMode::Normal;
            app.status_message = "Placeholder fill cancelled".into();
        }
        KeyCode::Enter | KeyCode::Tab => {
            if let Some(ref mut ps) = app.placeholder_state {
                ps.values.push(ps.current_input.clone());
                ps.current_input.clear();
                ps.current_index += 1;

                if ps.current_index >= ps.placeholders.len() {
                    // All placeholders filled — build final command and copy
                    let final_cmd =
                        placeholders::fill_placeholders(&ps.template, &ps.placeholders, &ps.values);
                    app.placeholder_state = None;
                    copy_to_clipboard(app, &final_cmd);
                } else if let Some(name) = ps.placeholders.get(ps.current_index) {
                    app.status_message = format!(
                        "Fill <{}> ({}/{})",
                        name,
                        ps.current_index + 1,
                        ps.placeholders.len()
                    );
                } else {
                    // Safety fallback - should not happen
                    app.placeholder_state = None;
                    app.input_mode = InputMode::Normal;
                    app.status_message = "Error: Invalid placeholder state".into();
                }
            }
        }
        KeyCode::Backspace => {
            if let Some(ref mut ps) = app.placeholder_state {
                ps.current_input.pop();
            }
        }
        KeyCode::Char(c) => {
            if let Some(ref mut ps) = app.placeholder_state {
                ps.current_input.push(c);
            }
        }
        _ => {}
    }
}

/// Initiates the copy flow.
/// - No placeholders → copies directly and shows confirm dialog.
/// - Has placeholders → shows choice dialog (raw vs fill).
fn initiate_copy(app: &mut App) {
    if let Some(item) = app.selected_item() {
        let found_placeholders = placeholders::extract_placeholders(&item.command);

        if found_placeholders.is_empty() {
            copy_to_clipboard(app, &item.command);
        } else {
            app.input_mode = InputMode::CopyChoice;
            app.status_message =
                "[1] Copy raw (with <placeholders>) │ [2] Fill variables │ [Esc] Cancel".into();
        }
    }
}

/// Copies the given string to clipboard and transitions to the CopiedConfirm dialog.
fn copy_to_clipboard(app: &mut App, text: &str) {
    // Try platform-specific clipboard first
    match crate::platform::copy_to_clipboard(text) {
        Ok(()) => {
            app.last_copied = text.to_string();
            app.input_mode = InputMode::CopiedConfirm;
            app.status_message = "✅ Copied to clipboard!".into();
            return;
        }
        Err(e) => {
            eprintln!("Platform clipboard failed: {}", e);
        }
    }

    // Fallback to copypasta crate
    match ClipboardContext::new() {
        Ok(mut ctx) => match ctx.set_contents(text.to_string()) {
            Ok(()) => {
                app.last_copied = text.to_string();
                app.input_mode = InputMode::CopiedConfirm;
                app.status_message = "✅ Copied to clipboard!".into();
            }
            Err(e) => {
                app.input_mode = InputMode::Normal;
                app.status_message = format!("❌ Failed to copy to clipboard: {}", e);
            }
        },
        Err(e) => {
            app.input_mode = InputMode::Normal;
            app.status_message = format!("❌ Failed to access clipboard: {}", e);
        }
    }
}
