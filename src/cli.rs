use std::time::{SystemTime, UNIX_EPOCH};

use clap::Parser;
use copypasta::{ClipboardContext, ClipboardProvider};

use crate::models::CommandItem;
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
}

impl Cli {
    /// Returns true if any headless CLI flag was provided.
    pub fn is_headless(&self) -> bool {
        self.search.is_some() || self.copy.is_some() || self.list || self.add.is_some()
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


