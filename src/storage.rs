use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use crate::constants::VAULT_FILENAME;
use crate::models::CommandItem;

/// Returns the path to the vault's JSON storage file (~/.cmd-vault.json).
pub fn get_file_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(VAULT_FILENAME);
    path
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
