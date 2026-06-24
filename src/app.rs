use ratatui::widgets::ListState;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::constants::{
    DEFAULT_STATUS, DEFAULT_TIMESTAMP, MAX_COMMAND_LENGTH, MAX_DESC_LENGTH, MAX_NAME_LENGTH,
};
use crate::models::CommandItem;
use crate::storage;
/// Available sort strategies (re-exported from utils).
pub use crate::utils::SortMode;
use crate::utils::{find_matching_items_owned, has_duplicate_name, sort_items};

/// The active input/interaction mode of the application.
#[derive(PartialEq)]
pub enum InputMode {
    Normal,
    Search,
    AddName,
    AddCommand,
    AddDesc,
    ConfirmDelete,
    FillPlaceholder,
    /// Prompt user to choose: copy raw (with placeholders) or fill them in.
    CopyChoice,
    /// Confirmation dialog after a successful copy.
    CopiedConfirm,
    /// Sort picker modal.
    SortPicker,
    /// Expanded detail view of the selected item.
    ExpandedView,
}

/// Holds state for the interactive placeholder fill-in flow.
pub struct PlaceholderState {
    /// The original command template (with `<placeholders>` intact).
    pub template: String,
    /// Ordered list of placeholder names extracted from the template.
    pub placeholders: Vec<String>,
    /// Values the user has entered so far (parallel to `placeholders`).
    pub values: Vec<String>,
    /// Index of the placeholder currently being filled.
    pub current_index: usize,
    /// The text being typed for the current placeholder.
    pub current_input: String,
}

/// Core application state.
pub struct App {
    pub items: Vec<CommandItem>,
    pub state: ListState,
    pub search_query: String,
    pub input_mode: InputMode,
    pub new_name: String,
    pub new_command: String,
    pub new_desc: String,
    pub status_message: String,
    pub placeholder_state: Option<PlaceholderState>,
    /// The command string that was last copied to clipboard (shown in confirm dialog).
    pub last_copied: String,
    /// Current active sort mode.
    pub sort_mode: SortMode,
    /// ListState for navigating search results in the right panel.
    pub search_results_state: ListState,
}

impl App {
    pub fn new() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));

        let items = storage::load_items().unwrap_or_else(|_| {
            vec![CommandItem {
                name: "FFmpeg Compress Video".into(),
                command: "ffmpeg -i input.mp4 -vcodec libx265 -crf 28 output.mp4".into(),
                desc: "Compresses mp4 video using the efficient H.265 codec to save space.".into(),
                created_at: DEFAULT_TIMESTAMP,
            }]
        });

        Self {
            items,
            state,
            search_query: String::new(),
            input_mode: InputMode::Normal,
            new_name: String::new(),
            new_command: String::new(),
            new_desc: String::new(),
            status_message: String::from(DEFAULT_STATUS),
            placeholder_state: None,
            last_copied: String::new(),
            sort_mode: SortMode::AZ,
            search_results_state: ListState::default(),
        }
    }

    /// Returns ALL items sorted by current mode (no search filter). Used for the sidebar.
    pub fn all_items_sorted(&self) -> Vec<CommandItem> {
        let mut results = self.items.clone();
        sort_items(&mut results, self.sort_mode);
        results
    }

    /// Returns only items matching the current search query, sorted. Used for search results panel.
    pub fn search_results(&self) -> Vec<CommandItem> {
        if self.search_query.is_empty() {
            return Vec::new();
        }
        let mut results = find_matching_items_owned(&self.items, &self.search_query);
        sort_items(&mut results, self.sort_mode);
        results
    }

    /// Returns true if there's an active search query.
    pub fn has_search(&self) -> bool {
        !self.search_query.is_empty()
    }

    /// Gets the currently selected item based on context (search results or main list).
    pub fn selected_item(&self) -> Option<CommandItem> {
        if self.has_search() {
            let results = self.search_results();
            self.search_results_state
                .selected()
                .and_then(|idx| results.get(idx).cloned())
        } else {
            let all = self.all_items_sorted();
            self.state.selected().and_then(|idx| all.get(idx).cloned())
        }
    }

    /// Removes the currently selected item. Returns the deleted item's name if successful.
    pub fn remove_current_selection(&mut self) -> Option<String> {
        let target_item = self.selected_item()?;
        if let Some(master_idx) = self
            .items
            .iter()
            .position(|x| x.name == target_item.name && x.command == target_item.command)
        {
            let deleted_name = self.items[master_idx].name.clone();
            self.items.remove(master_idx);

            // Add proper error handling for save operation
            if let Err(e) = storage::save_items(&self.items) {
                self.status_message = format!("⚠️ Failed to save changes: {}", e);
                return None;
            }

            // Adjust selection
            let all_len = self.all_items_sorted().len();
            if let Some(sel) = self.state.selected() {
                if all_len == 0 {
                    self.state.select(None);
                } else if sel >= all_len {
                    self.state.select(Some(all_len - 1));
                }
            }

            return Some(deleted_name);
        }
        None
    }

    /// Adds the new command from the form fields and saves to disk.
    /// Rejects duplicate names and validates input bounds.
    pub fn submit_new_command(&mut self) {
        if self.new_name.is_empty() || self.new_command.is_empty() {
            self.status_message = "⚠️ Name and Command are required fields!".into();
            return;
        }

        // Validate input length bounds
        if self.new_name.len() > MAX_NAME_LENGTH {
            self.status_message = format!("⚠️ Name too long (max {} chars)", MAX_NAME_LENGTH);
            return;
        }

        if self.new_command.len() > MAX_COMMAND_LENGTH {
            self.status_message = format!("⚠️ Command too long (max {} chars)", MAX_COMMAND_LENGTH);
            return;
        }

        if self.new_desc.len() > MAX_DESC_LENGTH {
            self.status_message =
                format!("⚠️ Description too long (max {} chars)", MAX_DESC_LENGTH);
            return;
        }

        // Basic input sanitization - remove control characters
        let sanitized_name = self
            .new_name
            .chars()
            .filter(|c| !c.is_control())
            .collect::<String>();
        let sanitized_command = self
            .new_command
            .chars()
            .filter(|c| !c.is_control())
            .collect::<String>();
        let sanitized_desc = self
            .new_desc
            .chars()
            .filter(|c| !c.is_control())
            .collect::<String>();

        if sanitized_name.is_empty() || sanitized_command.is_empty() {
            self.status_message = "⚠️ Invalid characters in name or command!".into();
            return;
        }

        // Enforce unique names (case-insensitive)
        if has_duplicate_name(&self.items, &sanitized_name) {
            self.status_message = format!(
                "⚠️ Name '{}' already exists! Choose a unique name.",
                sanitized_name
            );
            return;
        }

        self.items.push(CommandItem {
            name: sanitized_name,
            command: sanitized_command,
            desc: sanitized_desc,
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        });

        // Add proper error handling for save operation
        match storage::save_items(&self.items) {
            Ok(()) => {
                self.input_mode = InputMode::Normal;
                self.status_message =
                    "💾 Successfully synchronized configuration to local storage!".into();
            }
            Err(e) => {
                // Revert the addition if save failed
                self.items.pop();
                self.status_message = format!("⚠️ Failed to save command: {}", e);
            }
        }
    }

    /// Move selection to the next item (wraps around).
    pub fn next(&mut self) {
        if self.has_search() {
            let len = self.search_results().len();
            if len == 0 {
                self.search_results_state.select(None);
                return;
            }
            let i = match self.search_results_state.selected() {
                Some(i) => {
                    if i >= len - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.search_results_state.select(Some(i));
        } else {
            let len = self.all_items_sorted().len();
            if len == 0 {
                self.state.select(None);
                return;
            }
            let i = match self.state.selected() {
                Some(i) => {
                    if i >= len - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.state.select(Some(i));
        }
    }

    /// Move selection to the previous item (wraps around).
    pub fn previous(&mut self) {
        if self.has_search() {
            let len = self.search_results().len();
            if len == 0 {
                self.search_results_state.select(None);
                return;
            }
            let i = match self.search_results_state.selected() {
                Some(i) => {
                    if i == 0 {
                        len - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            self.search_results_state.select(Some(i));
        } else {
            let len = self.all_items_sorted().len();
            if len == 0 {
                self.state.select(None);
                return;
            }
            let i = match self.state.selected() {
                Some(i) => {
                    if i == 0 {
                        len - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            self.state.select(Some(i));
        }
    }

    /// Applies a new sort mode and persists the reordered list.
    pub fn apply_sort(&mut self, mode: SortMode) {
        self.sort_mode = mode;
        self.input_mode = InputMode::Normal;
        self.status_message = format!("Sorted: {}", mode.label());
    }

    /// Ensures the selection index stays within bounds.
    pub fn clamp_selection(&mut self) {
        // Clamp main list
        let total = self.all_items_sorted().len();
        if let Some(selected) = self.state.selected() {
            if total == 0 {
                self.state.select(None);
            } else if selected >= total {
                self.state.select(Some(total - 1));
            }
        } else if total > 0 {
            self.state.select(Some(0));
        }

        // Clamp search results
        if self.has_search() {
            let search_total = self.search_results().len();
            if let Some(selected) = self.search_results_state.selected() {
                if search_total == 0 {
                    self.search_results_state.select(None);
                } else if selected >= search_total {
                    self.search_results_state.select(Some(search_total - 1));
                }
            } else if search_total > 0 {
                self.search_results_state.select(Some(0));
            }
        } else {
            self.search_results_state.select(None);
        }
    }
}
