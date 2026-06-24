/// Shared utilities for search, sorting, and validation logic.

use crate::models::CommandItem;

/// Available sort strategies.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SortMode {
    /// Alphabetical A → Z by name.
    AZ,
    /// Reverse alphabetical Z → A by name.
    ZA,
    /// Newest first (by created_at timestamp).
    NewestFirst,
    /// Oldest first (by created_at timestamp).
    OldestFirst,
    /// Shortest command string first.
    ShortestFirst,
}

impl SortMode {
    pub fn label(self) -> &'static str {
        match self {
            SortMode::AZ => "A → Z (alphabetical)",
            SortMode::ZA => "Z → A (reverse)",
            SortMode::NewestFirst => "Newest first",
            SortMode::OldestFirst => "Oldest first",
            SortMode::ShortestFirst => "Shortest command first",
        }
    }
}

/// Finds items matching a query (case-insensitive, checks name and command).
/// Returns references to matching items for efficiency.
pub fn find_matching_items<'a>(items: &'a [CommandItem], query: &str) -> Vec<&'a CommandItem> {
    if query.is_empty() {
        return Vec::new();
    }
    
    let query_lower = query.to_lowercase();
    items
        .iter()
        .filter(|item| {
            item.name.to_lowercase().contains(&query_lower)
                || item.command.to_lowercase().contains(&query_lower)
        })
        .collect()
}

/// Finds items matching a query and returns owned copies (for when you need to modify or store the results).
pub fn find_matching_items_owned(items: &[CommandItem], query: &str) -> Vec<CommandItem> {
    if query.is_empty() {
        return Vec::new();
    }
    
    let query_lower = query.to_lowercase();
    items
        .iter()
        .filter(|item| {
            item.name.to_lowercase().contains(&query_lower)
                || item.command.to_lowercase().contains(&query_lower)
        })
        .cloned()
        .collect()
}

/// Sorts items according to the specified sort mode.
/// Modifies the vector in place for efficiency.
pub fn sort_items(items: &mut Vec<CommandItem>, mode: SortMode) {
    match mode {
        SortMode::AZ => items.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase())),
        SortMode::ZA => items.sort_by(|a, b| b.name.to_lowercase().cmp(&a.name.to_lowercase())),
        SortMode::NewestFirst => items.sort_by(|a, b| b.created_at.cmp(&a.created_at)),
        SortMode::OldestFirst => items.sort_by(|a, b| a.created_at.cmp(&b.created_at)),
        SortMode::ShortestFirst => items.sort_by(|a, b| a.command.len().cmp(&b.command.len())),
    }
}



/// Checks if a name already exists in the items list (case-insensitive).
/// Returns true if a duplicate is found.
pub fn has_duplicate_name(items: &[CommandItem], name: &str) -> bool {
    let name_lower = name.to_lowercase();
    items
        .iter()
        .any(|item| item.name.to_lowercase() == name_lower)
}



#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_items() -> Vec<CommandItem> {
        vec![
            CommandItem {
                name: "Zebra Command".to_string(),
                command: "zebra --help".to_string(),
                desc: "A test command".to_string(),
                created_at: 1000,
            },
            CommandItem {
                name: "Alpha Command".to_string(),
                command: "alpha --version".to_string(),
                desc: "Another test".to_string(),
                created_at: 2000,
            },
            CommandItem {
                name: "Beta Long Command".to_string(),
                command: "beta --very-long-command-with-many-options".to_string(),
                desc: "Long command".to_string(),
                created_at: 1500,
            },
        ]
    }

    #[test]
    fn test_find_matching_items() {
        let items = create_test_items();
        
        // Test search by name
        let results = find_matching_items(&items, "alpha");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Alpha Command");
        
        // Test search by command
        let results = find_matching_items(&items, "zebra");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Zebra Command");
        
        // Test case-insensitive
        let results = find_matching_items(&items, "ALPHA");
        assert_eq!(results.len(), 1);
        
        // Test empty query
        let results = find_matching_items(&items, "");
        assert_eq!(results.len(), 0);
        
        // Test no matches
        let results = find_matching_items(&items, "nonexistent");
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_sort_items() {
        let mut items = create_test_items();
        
        // Test A-Z sorting
        sort_items(&mut items, SortMode::AZ);
        assert_eq!(items[0].name, "Alpha Command");
        assert_eq!(items[1].name, "Beta Long Command");
        assert_eq!(items[2].name, "Zebra Command");
        
        // Test newest first
        sort_items(&mut items, SortMode::NewestFirst);
        assert_eq!(items[0].created_at, 2000); // Alpha (newest)
        assert_eq!(items[1].created_at, 1500); // Beta
        assert_eq!(items[2].created_at, 1000); // Zebra (oldest)
    }

    #[test]
    fn test_has_duplicate_name() {
        let items = create_test_items();
        
        // Test existing name (case-insensitive)
        assert!(has_duplicate_name(&items, "alpha command"));
        assert!(has_duplicate_name(&items, "ZEBRA COMMAND"));
        
        // Test non-existing name
        assert!(!has_duplicate_name(&items, "New Command"));
    }
}