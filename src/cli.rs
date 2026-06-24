use std::time::{SystemTime, UNIX_EPOCH};

use clap::Parser;
use copypasta::{ClipboardContext, ClipboardProvider};

use crate::models::CommandItem;
use crate::platform;
use crate::storage;
use crate::utils::{find_matching_items, has_duplicate_name};

#[derive(Parser)]
#[command(
    name = "cmd-vault",
    about = "📦 Terminal command vault — save, search, and copy commands instantly",
    version
)]
pub struct Cli {
    /// Search vault and print matching results
    #[arg(short, long)]
    pub search: Option<String>,

    /// Copy first matching command to clipboard
    #[arg(short, long)]
    pub copy: Option<String>,

    /// List all saved commands
    #[arg(short, long)]
    pub list: bool,

    /// Add a new command: -a "name" "command" ["description"]
    #[arg(short, long, num_args = 2..=3)]
    pub add: Option<Vec<String>>,

    /// Show system information and requirements
    #[arg(long)]
    pub system_info: bool,
}

impl Cli {
    /// Returns true if any headless CLI flag was provided.
    pub fn is_headless(&self) -> bool {
        self.search.is_some() || self.copy.is_some() || self.list || self.add.is_some() || self.system_info
    }
}

/// Runs the headless CLI operation and exits.
pub fn run_headless(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(ref query) = cli.search {
        cmd_search(query)?;
    } else if let Some(ref query) = cli.copy {
        cmd_copy(query)?;
    } else if cli.list {
        cmd_list()?;
    } else if let Some(ref args) = cli.add {
        cmd_add(args)?;
    } else if cli.system_info {
        cmd_system_info()?;
    }
    Ok(())
}

/// Search and print matching results to stdout.
fn cmd_search(query: &str) -> Result<(), Box<dyn std::error::Error>> {
    let items = storage::load_items().unwrap_or_default();
    let results = find_matching_items(&items, query);

    if results.is_empty() {
        println!("  No matches for '{}'", query);
        return Ok(());
    }

    for item in &results {
        println!("┌─ \x1b[1;36m{}\x1b[0m", item.name);
        println!("│  \x1b[1;37m{}\x1b[0m", item.command);
        if !item.desc.is_empty() {
            println!("│  \x1b[2m{}\x1b[0m", item.desc);
        }
        println!("└─");
    }
    println!("\n  {} result(s) found", results.len());
    Ok(())
}

/// Copy first matching command to clipboard.
fn cmd_copy(query: &str) -> Result<(), Box<dyn std::error::Error>> {
    let items = storage::load_items().unwrap_or_default();
    let results = find_matching_items(&items, query);

    if let Some(first) = results.first() {
        let mut ctx = ClipboardContext::new()
            .map_err(|e| format!("Failed to access clipboard: {}", e))?;
        ctx.set_contents(first.command.clone())
            .map_err(|e| format!("Failed to set clipboard: {}", e))?;
        println!("✅ Copied to clipboard:");
        println!("   \x1b[1;36m{}\x1b[0m │ \x1b[1;37m{}\x1b[0m", first.name, first.command);
    } else {
        eprintln!("  No match for '{}'", query);
        std::process::exit(1);
    }
    Ok(())
}

/// List all saved commands.
fn cmd_list() -> Result<(), Box<dyn std::error::Error>> {
    let items = storage::load_items().unwrap_or_default();

    if items.is_empty() {
        println!("  Vault is empty. Use `cmd-vault -a` or the TUI to add commands.");
        return Ok(());
    }

    println!("  📦 CmdVault ({} commands)\n", items.len());
    for (i, item) in items.iter().enumerate() {
        println!(
            "  \x1b[2m{:>3}.\x1b[0m \x1b[1;36m{}\x1b[0m",
            i + 1,
            item.name
        );
        println!("       \x1b[37m{}\x1b[0m", item.command);
        if !item.desc.is_empty() {
            println!("       \x1b[2m{}\x1b[0m", item.desc);
        }
        println!();
    }
    Ok(())
}

/// Add a new command from CLI args.
fn cmd_add(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let name = &args[0];
    let command = &args[1];
    let desc = args.get(2).cloned().unwrap_or_default();

    let mut items = storage::load_items().unwrap_or_default();

    // Enforce unique names
    if has_duplicate_name(&items, name) {
        eprintln!("  ⚠️  Name '{}' already exists. Choose a unique name.", name);
        std::process::exit(1);
    }

    items.push(CommandItem {
        name: name.clone(),
        command: command.clone(),
        desc,
        created_at: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
    });

    storage::save_items(&items)?;
    println!("✅ Added: \x1b[1;36m{}\x1b[0m", name);
    println!("   \x1b[37m{}\x1b[0m", command);
    Ok(())
}

/// Show system information and platform compatibility.
fn cmd_system_info() -> Result<(), Box<dyn std::error::Error>> {
    let platform_info = platform::get_platform_info();
    let system_check = platform::check_system_requirements();
    let shortcuts = platform::get_shortcuts();

    println!("🖥️  \x1b[1;36mCmdVault System Information\x1b[0m");
    println!("   Platform: {} ({})", platform_info.name, platform_info.arch);
    println!("   Config file: {}", storage::get_file_path().display());
    println!("   Clipboard method: {}", platform_info.clipboard_method);
    
    println!("\n📋 \x1b[1;33mClipboard Support\x1b[0m");
    println!("   {}", system_check.clipboard_message);
    
    if !system_check.recommendations.is_empty() {
        println!("\n💡 \x1b[1;35mRecommendations\x1b[0m");
        for rec in &system_check.recommendations {
            println!("   • {}", rec);
        }
    }

    println!("\n⌨️  \x1b[1;32mKeyboard Shortcuts\x1b[0m");
    println!("   Copy: {}", shortcuts.copy);
    println!("   Paste: {}", shortcuts.paste);  
    println!("   Quit: {}", shortcuts.quit);

    // Test clipboard functionality
    println!("\n🧪 \x1b[1;34mTesting Clipboard\x1b[0m");
    match platform::copy_to_clipboard("CmdVault clipboard test") {
        Ok(()) => {
            println!("   ✅ Platform clipboard working");
        }
        Err(e) => {
            println!("   ⚠️  Platform clipboard failed: {}", e);
            
            // Try fallback
            match ClipboardContext::new() {
                Ok(mut ctx) => {
                    match ctx.set_contents("CmdVault fallback test".to_string()) {
                        Ok(()) => println!("   ✅ Fallback clipboard working"),
                        Err(e2) => println!("   ❌ All clipboard methods failed: {}", e2),
                    }
                }
                Err(e2) => {
                    println!("   ❌ Fallback clipboard unavailable: {}", e2);
                }
            }
        }
    }

    Ok(())
}


