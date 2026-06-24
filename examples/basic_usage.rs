// Example: Basic usage of cmd-vault as a library
use cmd_vault::models::CommandItem;
use cmd_vault::storage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create some example commands
    let commands = vec![
        CommandItem {
            name: "Git Status".to_string(),
            command: "git status --short".to_string(),
            desc: "Show git repository status in short format".to_string(),
            created_at: 1700000000,
        },
        CommandItem {
            name: "Docker Cleanup".to_string(),
            command: "docker system prune -f".to_string(),
            desc: "Clean up unused Docker resources".to_string(),
            created_at: 1700000001,
        },
        CommandItem {
            name: "Find Large Files".to_string(),
            command: "find . -type f -size +100M -exec ls -lh {} \\;".to_string(),
            desc: "Find files larger than 100MB in current directory".to_string(),
            created_at: 1700000002,
        },
    ];

    // Show the storage file path
    let path = storage::get_file_path();
    println!("📂 Storage path: {}", path.display());

    // Display example commands
    println!("\n📋 Example commands:");
    for (i, cmd) in commands.iter().enumerate() {
        println!("  {}. {} - {}", i + 1, cmd.name, cmd.desc);
        println!("     Command: {}", cmd.command);
    }

    // Demonstrate serialization
    let json = serde_json::to_string_pretty(&commands)?;
    println!("\n💾 Serialized as JSON ({} bytes)", json.len());

    println!("\n✅ Done! In the full app, use `cmd-vault` to manage commands interactively.");

    Ok(())
}
