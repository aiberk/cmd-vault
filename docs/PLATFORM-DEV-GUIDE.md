# 🤝 Platform Development Guide

## 👥 Team Development Structure

### **You (Linux/macOS Developer):**
- ✅ Maintain `LinuxPlatform` and `MacOSPlatform` implementations
- ✅ Handle all core application logic (UI, storage, etc.)
- ✅ Review Windows PRs for integration

### **Windows Developer:**
- 🎯 **ONLY** needs to work on `WindowsPlatform` implementation in `src/platform.rs`
- 🎯 Focus on lines 200-300 in the platform module
- 🎯 Test Windows-specific functionality

## 📁 What Each Developer Touches

### Core Files (You maintain):
```
src/
├── main.rs              ✅ Entry point
├── app.rs               ✅ Application logic  
├── cli.rs               ✅ CLI interface
├── handler.rs           ✅ Event handling
├── ui.rs                ✅ Terminal UI
├── models.rs            ✅ Data structures
├── storage.rs           ✅ File operations
├── utils.rs             ✅ Search/sort logic
├── placeholders.rs      ✅ Variable system
└── constants.rs         ✅ App constants
```

### Platform File (Shared):
```
src/platform.rs
├── Lines 1-150      ✅ Trait definitions & shared code (You)
├── Lines 150-200    ✅ Linux implementation (You)  
├── Lines 200-250    ✅ macOS implementation (You)
└── Lines 250-300    🎯 Windows implementation (Windows dev)
```

## 🎯 Windows Developer Quick Start

### 1. Setup (5 minutes)
```bash
# Clone the repo
git clone <repo-url>
cd cmd-vault

# Install Rust (if needed)
# https://rustup.rs/

# Build and test
cargo build
cargo run -- --system-info
```

### 2. Your Mission 🎯
**Edit ONLY the `WindowsPlatform` implementation in `src/platform.rs`**

Find this section around line 250:
```rust
#[cfg(target_os = "windows")]
impl PlatformOps for WindowsPlatform {
    // TODO: You implement these 5 functions
}
```

### 3. Functions to Implement

#### **Function 1: `get_config_dir()`**
```rust
fn get_config_dir() -> Result<PathBuf, PlatformError> {
    // GOAL: Return path to %APPDATA%\cmd-vault\vault.json
    // FALLBACK: %USERPROFILE%\.cmd-vault.json
    
    // HINT: Use dirs::config_dir() first
    // HINT: Create directory with std::fs::create_dir_all()
    // HINT: Return the full file path, not just directory
}
```

#### **Function 2: `copy_to_clipboard()`** ⭐ MOST IMPORTANT
```rust
fn copy_to_clipboard(text: &str) -> Result<(), PlatformError> {
    // GOAL: Copy text to Windows clipboard
    
    // OPTION 1: Use clip.exe command (easiest)
    // OPTION 2: Use Windows API directly
    // OPTION 3: Try both with fallback
    
    // CURRENT CODE: Already has basic clip.exe - improve it!
}
```

#### **Function 3: `get_shortcuts()`**
```rust
fn get_shortcuts() -> PlatformShortcuts {
    // GOAL: Return Windows-appropriate keyboard shortcuts
    // HINT: copy: "Ctrl+C", paste: "Ctrl+V", quit: "Alt+F4"
}
```

#### **Function 4: `check_system_requirements()`**
```rust
fn check_system_requirements() -> SystemCheck {
    // GOAL: Check what clipboard tools are available on Windows
    // HINT: Test if clip.exe works
    // HINT: Maybe check PowerShell clipboard commands
    // HINT: Return helpful error messages for users
}
```

#### **Function 5: `get_platform_info()`**
```rust
fn get_platform_info() -> PlatformInfo {
    // GOAL: Return Windows platform identification
    // HINT: name: "Windows", clipboard_method: "clip.exe"
}
```

### 4. Testing Your Work

```bash
# Test basic functionality
cargo run -- --system-info

# Test clipboard
cargo run -- -c "ffmpeg"  # Should copy to clipboard

# Test config location  
cargo run -- -a "Test" "echo test" "Test command"
# Should save to %APPDATA%\cmd-vault\vault.json
```

### 5. Advanced Windows Features (Optional)

#### PowerShell Clipboard Alternative:
```rust
// Alternative to clip.exe
Command::new("powershell")
    .args(["-Command", &format!("Set-Clipboard -Value '{}'", text)])
    .output()?;
```

#### Windows API Direct Access:
```rust
// For advanced developers - direct Windows API
// Requires winapi crate in Cargo.toml
use winapi::um::winuser::{OpenClipboard, SetClipboardData};
```

#### Windows-Specific Config Locations:
```rust
// Try multiple Windows locations in order:
// 1. %APPDATA%\cmd-vault\
// 2. %LOCALAPPDATA%\cmd-vault\  
// 3. %USERPROFILE%\.cmd-vault.json
```

## 🔄 Development Workflow

### Windows Developer Workflow:
1. **Fork/Clone** the repository
2. **Edit** only `WindowsPlatform` in `src/platform.rs`  
3. **Test** with `cargo run -- --system-info`
4. **Submit PR** with your changes
5. **We handle** integration testing on Linux/macOS

### Integration Process:
```
Windows Dev PR → Your Review → Merge → CI Tests All Platforms
```

## 🧪 Testing Strategy

### Local Testing (Windows Developer):
```bash
# Basic functionality
cargo test
cargo run

# System check
cargo run -- --system-info

# CLI operations  
cargo run -- -l
cargo run -- -a "Test" "echo test" "Description"
cargo run -- -s "test"
cargo run -- -c "test"

# TUI mode
cargo run
# Try copying commands (y key)
```

### Cross-Platform CI (Automatic):
- **GitHub Actions** builds Windows/Linux/macOS
- **All platforms** test clipboard functionality
- **Integration tests** verify compatibility

## 🚨 Common Windows Issues & Solutions

### Issue 1: Clipboard Not Working
```rust
// Make sure clip.exe is available
let output = Command::new("clip").arg("/?").output()?;
if !output.status.success() {
    return Err(PlatformError::Clipboard("clip.exe not found".to_string()));
}
```

### Issue 2: Path Issues
```rust  
// Use raw strings for Windows paths
r"C:\Users\username\AppData\Roaming\cmd-vault"

// Or use PathBuf for cross-platform safety
config_dir.join("cmd-vault").join("vault.json")
```

### Issue 3: Permission Errors
```rust
// Always check if directories are writable
if let Err(e) = std::fs::create_dir_all(&app_dir) {
    return Err(PlatformError::ConfigDir(format!("Cannot create directory: {}", e)));
}
```

## 📞 Communication

### Questions/Issues:
- **File GitHub Issues** for Windows-specific problems
- **Tag @your-username** for platform integration questions
- **Test locally first** before submitting PRs

### PR Template for Windows Developer:
```markdown
## Windows Platform Implementation

### Changes:
- [ ] `get_config_dir()` - Uses %APPDATA%\cmd-vault\
- [ ] `copy_to_clipboard()` - Uses clip.exe with fallbacks
- [ ] `check_system_requirements()` - Tests Windows tools
- [ ] `get_shortcuts()` - Windows keyboard shortcuts  
- [ ] `get_platform_info()` - Platform identification

### Testing Done:
- [ ] `cargo run -- --system-info` shows correct info
- [ ] `cargo run -- -c "test"` copies to clipboard
- [ ] Config saves to correct Windows location
- [ ] All tests pass: `cargo test`

### Notes:
(Any Windows-specific considerations or issues found)
```

## 🎯 Success Criteria

When the Windows implementation is complete:

✅ **Config files** save to appropriate Windows directories  
✅ **Clipboard operations** work reliably on Windows  
✅ **System info** shows Windows-specific information  
✅ **Error messages** are helpful for Windows users  
✅ **All CLI commands** work identically to Linux/macOS  
✅ **TUI interface** works in Windows Terminal/PowerShell  

This architecture lets you focus on what you know (Linux/macOS) while the Windows developer focuses on Windows-specific details. The platform abstraction keeps everything clean and maintainable!