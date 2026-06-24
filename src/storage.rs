use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use crate::models::CommandItem;
use crate::platform;

/// Returns the path to the vault's JSON storage file using platform-appropriate location.
pub fn get_file_path() -> PathBuf {
    // Use platform-specific config directory, fallback to old behavior
    let new_path = platform::get_config_dir().unwrap_or_else(|_| {
        // Fallback to home directory with original filename
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".cmd-vault.json")
    });

    // Migration: if old file exists but new doesn't, try to migrate
    let old_path = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".cmd-vault.json");

    if old_path.exists() && !new_path.exists() {
        if let Err(e) = migrate_config(&old_path, &new_path) {
            eprintln!("Warning: Could not migrate config file: {}", e);
            return old_path; // Fall back to old location
        }
    }

    new_path
}

/// Migrate config file from old location to new platform-appropriate location
fn migrate_config(old_path: &std::path::Path, new_path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    // Create new directory if needed
    if let Some(parent) = new_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Copy file contents
    std::fs::copy(old_path, new_path)?;
    
    println!("✅ Migrated config from {} to {}", old_path.display(), new_path.display());
    
    // Optionally remove old file (commented out for safety)
    // std::fs::remove_file(old_path)?;
    
    Ok(())
}

/// Loads command items from disk. Returns an error if the file doesn't exist or is malformed.
pub fn load_items() -> Result<Vec<CommandItem>, Box<dyn std::error::Error>> {
    let path = get_file_path();
    if !path.exists() {
        return Err("File does not exist".into());
    }
    let file = File::open(path)?;
    let items = serde_json::from_reader(file)?;
    Ok(items)
}

/// Persists the given command items to disk as pretty-printed JSON.
pub fn save_items(items: &[CommandItem]) -> Result<(), Box<dyn std::error::Error>> {
    let path = get_file_path();
    let mut file = File::create(path)?;
    let json = serde_json::to_string_pretty(items)?;
    file.write_all(json.as_bytes())?;
    file.flush()?; // Ensure data is written to disk
    Ok(())
}
