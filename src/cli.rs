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

    /// Show manual with all commands and usage examples
    #[arg(short = 'm', long)]
    pub manual: bool,
}

impl Cli {
    /// Returns true if any headless CLI flag was provided.
    pub fn is_headless(&self) -> bool {
        self.search.is_some()
            || self.copy.is_some()
            || self.list
            || self.add.is_some()
            || self.system_info
            || self.manual
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
    } else if cli.manual {
        cmd_manual()?;
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
        let mut ctx =
            ClipboardContext::new().map_err(|e| format!("Failed to access clipboard: {}", e))?;
        ctx.set_contents(first.command.clone())
            .map_err(|e| format!("Failed to set clipboard: {}", e))?;
        println!("✅ Copied to clipboard:");
        println!(
            "   \x1b[1;36m{}\x1b[0m │ \x1b[1;37m{}\x1b[0m",
            first.name, first.command
        );
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
        eprintln!(
            "  ⚠️  Name '{}' already exists. Choose a unique name.",
            name
        );
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
    println!(
        "   Platform: {} ({})",
        platform_info.name, platform_info.arch
    );
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
                Ok(mut ctx) => match ctx.set_contents("CmdVault fallback test".to_string()) {
                    Ok(()) => println!("   ✅ Fallback clipboard working"),
                    Err(e2) => println!("   ❌ All clipboard methods failed: {}", e2),
                },
                Err(e2) => {
                    println!("   ❌ Fallback clipboard unavailable: {}", e2);
                }
            }
        }
    }

    Ok(())
}
/// Show comprehensive manual with all commands and usage examples.
fn cmd_manual() -> Result<(), Box<dyn std::error::Error>> {
    println!("📦 \x1b[1;36mCmdVault Manual\x1b[0m");
    println!("   \x1b[2mA blazingly fast terminal command manager\x1b[0m\n");

    // Quick Start
    println!("🚀 \x1b[1;33mQuick Start\x1b[0m");
    println!("   cmd-vault                    Launch interactive TUI");
    println!("   cmd-vault -h                 Show help");
    println!("   cmd-vault -m                 Show this manual");
    println!();

    // CLI Commands
    println!("⚡ \x1b[1;32mCLI Commands\x1b[0m");
    println!();
    println!("  \x1b[1;37m🔍 SEARCH\x1b[0m");
    println!("    cmd-vault -s \"query\"         Search commands by name/content");
    println!("    cmd-vault --search \"ffmpeg\"  Same as above");
    println!();
    println!("    \x1b[2mExamples:\x1b[0m");
    println!("      cmd-vault -s \"docker\"       # Find Docker commands");
    println!("      cmd-vault -s \"compress\"     # Find compression tools");
    println!("      cmd-vault -s \"ssh\"          # Find SSH-related commands");
    println!();

    println!("  \x1b[1;37m📋 COPY\x1b[0m");
    println!("    cmd-vault -c \"query\"         Copy first match to clipboard");
    println!("    cmd-vault --copy \"ffmpeg\"    Same as above");
    println!();
    println!("    \x1b[2mExamples:\x1b[0m");
    println!("      cmd-vault -c \"ffmpeg\"       # Copy ffmpeg command");
    println!("      cmd-vault -c \"git reset\"    # Copy git reset command");
    println!();

    println!("  \x1b[1;37m📝 LIST\x1b[0m");
    println!("    cmd-vault -l                 List all saved commands");
    println!("    cmd-vault --list             Same as above");
    println!();

    println!("  \x1b[1;37m➕ ADD\x1b[0m");
    println!("    cmd-vault -a \"name\" \"cmd\" \"desc\"  Add new command");
    println!("    cmd-vault --add \"name\" \"cmd\"       Add without description");
    println!();
    println!("    \x1b[2mExamples:\x1b[0m");
    println!("      cmd-vault -a \"Git Status\" \"git status\" \"Check repo status\"");
    println!("      cmd-vault -a \"List Files\" \"ls -la\"");
    println!();

    println!("  \x1b[1;37m🔧 SYSTEM\x1b[0m");
    println!("    cmd-vault --system-info      Show platform & clipboard status");
    println!("    cmd-vault -m                 Show this manual");
    println!("    cmd-vault --manual           Same as above");
    println!();

    // TUI Controls
    println!("🎮 \x1b[1;34mTUI (Interactive Mode)\x1b[0m");
    println!();
    println!("  \x1b[1;37mNavigation:\x1b[0m");
    println!("    ↑/↓ or j/k              Navigate up/down");
    println!("    Enter                   Expand command details");
    println!("    q or Esc                Quit application");
    println!();
    println!("  \x1b[1;37mActions:\x1b[0m");
    println!("    /                       Start search mode");
    println!("    y                       Copy command to clipboard");
    println!("    a                       Add new command");
    println!("    d                       Delete selected command");
    println!("    s                       Sort commands");
    println!();
    println!("  \x1b[1;37mSearch Mode:\x1b[0m");
    println!("    Type to search          Filter commands in real-time");
    println!("    ↑/↓                     Navigate search results");
    println!("    Enter                   Expand selected result");
    println!("    Esc                     Exit search mode");
    println!();

    // Interactive Variables
    println!("🔧 \x1b[1;35mInteractive Variables\x1b[0m");
    println!();
    println!("  Commands can contain placeholders like \x1b[33m<variable>\x1b[0m:");
    println!(
        "    scp \x1b[33m<local_file>\x1b[0m \x1b[33m<user>\x1b[0m@\x1b[33m<host>\x1b[0m:\x1b[33m<remote_path>\x1b[0m"
    );
    println!();
    println!("  When copying, you'll be prompted to fill each variable:");
    println!("    Fill \x1b[33m<local_file>\x1b[0m (1/4): myfile.txt");
    println!("    Fill \x1b[33m<user>\x1b[0m (2/4): admin");
    println!("    Fill \x1b[33m<host>\x1b[0m (3/4): server.com");
    println!("    Fill \x1b[33m<remote_path>\x1b[0m (4/4): /home/admin/");
    println!();
    println!("  Result: scp myfile.txt admin@server.com:/home/admin/");
    println!();

    // Sort Options
    println!("📊 \x1b[1;36mSort Options\x1b[0m (Press 's' in TUI)");
    println!("    1. A → Z (alphabetical)");
    println!("    2. Z → A (reverse alphabetical)");
    println!("    3. Newest first (by creation date)");
    println!("    4. Oldest first (by creation date)");
    println!("    5. Shortest command first (by length)");
    println!();

    // File Locations
    println!("📁 \x1b[1;32mConfig File Locations\x1b[0m");
    let config_path = storage::get_file_path();
    println!("    Current: {}", config_path.display());

    let platform_info = platform::get_platform_info();
    match platform_info.name {
        "Linux" => {
            println!("    Standard: ~/.config/cmd-vault/vault.json");
            println!("    Fallback: ~/.cmd-vault.json");
        }
        "macOS" => {
            println!("    Standard: ~/Library/Application Support/cmd-vault/vault.json");
            println!("    Fallback: ~/.cmd-vault.json");
        }
        "Windows" => {
            println!("    Standard: %APPDATA%\\cmd-vault\\vault.json");
            println!("    Fallback: %USERPROFILE%\\.cmd-vault.json");
        }
        _ => {}
    }
    println!();

    // Practical Examples
    println!("💡 \x1b[1;33mPractical Examples\x1b[0m");
    println!();
    println!("  \x1b[1;37mDaily Workflow:\x1b[0m");
    println!("    cmd-vault -s \"git\"          # Find git commands");
    println!("    cmd-vault -c \"git status\"   # Copy git status");
    println!("    # Paste in terminal and run");
    println!();
    println!("  \x1b[1;37mSystem Administration:\x1b[0m");
    println!("    cmd-vault -a \"Disk Usage\" \"df -h\" \"Check disk space\"");
    println!("    cmd-vault -a \"Process List\" \"ps aux | grep <process>\"");
    println!("    cmd-vault -c \"disk\"         # Quick copy disk command");
    println!();
    println!("  \x1b[1;37mDevelopment:\x1b[0m");
    println!("    cmd-vault -a \"Docker Build\" \"docker build -t <image>:<tag> .\"");
    println!("    cmd-vault -a \"Run Tests\" \"npm test -- --coverage\"");
    println!("    cmd-vault -c \"docker\"       # Copy docker command");
    println!();

    // Keyboard Shortcuts
    let shortcuts = platform::get_shortcuts();
    println!(
        "⌨️  \x1b[1;34mKeyboard Shortcuts\x1b[0m ({})",
        platform_info.name
    );
    println!(
        "    Copy: {}              Paste: {}",
        shortcuts.copy, shortcuts.paste
    );
    println!(
        "    Quit: {}              Terminal varies by platform",
        shortcuts.quit
    );
    println!();

    // Tips & Tricks
    println!("🎯 \x1b[1;35mTips & Tricks\x1b[0m");
    println!("    • Use descriptive names for easy searching");
    println!("    • Add placeholders with \x1b[33m<variable>\x1b[0m for reusable commands");
    println!("    • Use the TUI for browsing, CLI for quick operations");
    println!("    • Sort by 'Newest' to find recently added commands");
    println!("    • Keep descriptions short but informative");
    println!();

    // Troubleshooting
    println!("🔧 \x1b[1;31mTroubleshooting\x1b[0m");
    println!("    Clipboard not working?    cmd-vault --system-info");
    println!("    Can't find commands?      cmd-vault -l");
    println!("    TUI not launching?        Check terminal compatibility");
    println!("    Permission errors?        Check config directory permissions");
    println!();

    // More Info
    println!("📚 \x1b[1;36mMore Information\x1b[0m");
    println!("    Help:                     cmd-vault --help");
    println!("    System Info:              cmd-vault --system-info");
    println!("    Version:                  cmd-vault --version");
    println!();
    println!("🎉 \x1b[1;32mHappy command managing!\x1b[0m");

    Ok(())
}
