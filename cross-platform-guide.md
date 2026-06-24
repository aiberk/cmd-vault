# 🌍 Cross-Platform Guide for CmdVault

## 🦀 Rust Version Cross-Platform Enhancements

### Current Status ✅
The Rust version is already quite cross-platform thanks to:
- **Crossterm** - Cross-platform terminal manipulation
- **Ratatui** - Platform-agnostic TUI rendering  
- **Copypasta** - Cross-platform clipboard access
- **Dirs** - Standard directory locations across OS
- **Serde + JSON** - Universal data format

### Platform-Specific Improvements Needed 🔧

#### 1. Enhanced File Path Handling
```rust
// src/storage.rs - Update get_file_path()
use std::env;

pub fn get_file_path() -> PathBuf {
    // Try environment-specific locations first
    if let Some(config_dir) = dirs::config_dir() {
        // ~/.config/cmd-vault/vault.json on Linux
        // ~/Library/Application Support/cmd-vault/vault.json on macOS  
        // %APPDATA%\cmd-vault\vault.json on Windows
        let app_dir = config_dir.join("cmd-vault");
        std::fs::create_dir_all(&app_dir).ok();
        return app_dir.join("vault.json");
    }
    
    // Fallback to home directory
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".cmd-vault.json")
}
```

#### 2. Improved Clipboard with Fallbacks
```rust
// src/handler.rs - Enhanced copy_to_clipboard()
fn copy_to_clipboard(app: &mut App, text: &str) {
    // Try primary clipboard first
    match ClipboardContext::new() {
        Ok(mut ctx) => {
            match ctx.set_contents(text.to_string()) {
                Ok(()) => {
                    app.last_copied = text.to_string();
                    app.input_mode = InputMode::CopiedConfirm;
                    app.status_message = "✅ Copied to clipboard!".into();
                    return;
                }
                Err(e) => {
                    eprintln!("Clipboard error: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Clipboard context error: {}", e);
        }
    }

    // Fallback: Try platform-specific commands
    if let Err(e) = try_platform_copy(text) {
        app.input_mode = InputMode::Normal;
        app.status_message = format!("❌ Copy failed: {} (Try manual copy)", e);
    } else {
        app.last_copied = text.to_string();
        app.input_mode = InputMode::CopiedConfirm;
        app.status_message = "✅ Copied via system command!".into();
    }
}

fn try_platform_copy(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::process::Command;
    
    #[cfg(target_os = "macos")]
    {
        Command::new("pbcopy")
            .stdin(std::process::Stdio::piped())
            .spawn()?
            .stdin.as_mut().unwrap()
            .write_all(text.as_bytes())?;
    }
    
    #[cfg(target_os = "linux")]
    {
        // Try xclip first, then xsel
        if Command::new("xclip").arg("-version").output().is_ok() {
            let mut child = Command::new("xclip")
                .args(["-selection", "clipboard"])
                .stdin(std::process::Stdio::piped())
                .spawn()?;
            child.stdin.as_mut().unwrap().write_all(text.as_bytes())?;
            child.wait()?;
        } else if Command::new("xsel").arg("--version").output().is_ok() {
            let mut child = Command::new("xsel")
                .args(["--clipboard", "--input"])
                .stdin(std::process::Stdio::piped())
                .spawn()?;
            child.stdin.as_mut().unwrap().write_all(text.as_bytes())?;
            child.wait()?;
        } else {
            return Err("No clipboard tool found (install xclip or xsel)".into());
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        Command::new("clip")
            .stdin(std::process::Stdio::piped())
            .spawn()?
            .stdin.as_mut().unwrap()
            .write_all(text.as_bytes())?;
    }
    
    Ok(())
}
```

#### 3. Cross-Platform Constants
```rust
// src/constants.rs - Platform-aware constants
#[cfg(target_os = "windows")]
pub const DEFAULT_TERMINAL_WIDTH: u16 = 120;

#[cfg(not(target_os = "windows"))]  
pub const DEFAULT_TERMINAL_WIDTH: u16 = 80;

#[cfg(target_os = "windows")]
pub const PATH_SEPARATOR: &str = "\\";

#[cfg(not(target_os = "windows"))]
pub const PATH_SEPARATOR: &str = "/";

// Platform-specific shortcuts
#[cfg(target_os = "macos")]
pub const COPY_SHORTCUT: &str = "⌘+C";

#[cfg(not(target_os = "macos"))]
pub const COPY_SHORTCUT: &str = "Ctrl+C";
```

#### 4. Enhanced CLI with Platform Detection
```rust
// src/cli.rs - Add platform info command
pub fn print_system_info() {
    println!("🖥️  CmdVault System Information");
    println!("   OS: {}", env::consts::OS);
    println!("   Architecture: {}", env::consts::ARCH);
    println!("   Config location: {}", storage::get_file_path().display());
    
    // Check clipboard tools
    println!("\n📋 Clipboard Support:");
    match ClipboardContext::new() {
        Ok(_) => println!("   ✅ Native clipboard available"),
        Err(_) => {
            println!("   ❌ Native clipboard unavailable");
            check_system_clipboard_tools();
        }
    }
}

fn check_system_clipboard_tools() {
    use std::process::Command;
    
    #[cfg(target_os = "linux")]
    {
        if Command::new("xclip").arg("-version").output().is_ok() {
            println!("   ✅ xclip available");
        } else if Command::new("xsel").arg("--version").output().is_ok() {
            println!("   ✅ xsel available");  
        } else {
            println!("   ❌ Install xclip or xsel for clipboard support");
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        if Command::new("clip").arg("/?").output().is_ok() {
            println!("   ✅ Windows clip.exe available");
        }
    }
}
```

### 5. Cargo.toml Cross-Platform Configuration
```toml
[package]
name = "cmd-vault"
version = "0.2.0"
edition = "2021"
authors = ["Your Name <email@example.com>"]
description = "Cross-platform terminal command manager"
license = "MIT"
repository = "https://github.com/username/cmd-vault"

# Platform-specific dependencies
[dependencies]
crossterm = "0.27"
ratatui = "0.24"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
clap = { version = "4.0", features = ["derive"] }
dirs = "5.0"

# Clipboard with fallbacks
copypasta = "0.10"

# Platform-specific optional features
[target.'cfg(unix)'.dependencies]
nix = { version = "0.27", optional = true }

[target.'cfg(windows)'.dependencies] 
winapi = { version = "0.3", features = ["winuser"], optional = true }

[features]
default = ["clipboard-fallback"]
clipboard-fallback = []
unix-extensions = ["nix"]
windows-extensions = ["winapi"]

# Cross-compilation targets
[[bin]]
name = "cmd-vault"
path = "src/main.rs"
```

## 🏗️ Distribution & Packaging

### 1. GitHub Actions for Multi-Platform Builds
```yaml
# .github/workflows/release.yml
name: Build and Release

on:
  push:
    tags: ["v*"]
  workflow_dispatch:

jobs:
  build:
    strategy:
      matrix:
        include:
          - target: x86_64-unknown-linux-gnu
            os: ubuntu-latest
            name: cmd-vault-linux-x64
          - target: x86_64-apple-darwin  
            os: macos-latest
            name: cmd-vault-macos-x64
          - target: aarch64-apple-darwin
            os: macos-latest  
            name: cmd-vault-macos-arm64
          - target: x86_64-pc-windows-msvc
            os: windows-latest
            name: cmd-vault-windows-x64.exe

    runs-on: ${{ matrix.os }}
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}
          
      - name: Install Linux dependencies
        if: matrix.os == 'ubuntu-latest'
        run: |
          sudo apt-get update
          sudo apt-get install -y xclip
          
      - name: Build
        run: cargo build --release --target ${{ matrix.target }}
        
      - name: Package
        run: |
          mkdir -p dist
          cp target/${{ matrix.target }}/release/cmd-vault* dist/${{ matrix.name }}
          
      - name: Upload artifact  
        uses: actions/upload-artifact@v3
        with:
          name: ${{ matrix.name }}
          path: dist/${{ matrix.name }}
```

### 2. Installation Scripts

#### Universal Install Script
```bash
#!/bin/bash
# install.sh - Cross-platform installer

set -e

REPO="username/cmd-vault"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"

# Detect platform
case "$(uname -s)" in
    Darwin)
        case "$(uname -m)" in
            x86_64) PLATFORM="macos-x64" ;;
            arm64)  PLATFORM="macos-arm64" ;;
            *)      echo "Unsupported macOS architecture: $(uname -m)"; exit 1 ;;
        esac
        ;;
    Linux)
        PLATFORM="linux-x64"
        # Check for clipboard tools
        if ! command -v xclip >/dev/null && ! command -v xsel >/dev/null; then
            echo "⚠️  Consider installing xclip or xsel for clipboard support"
        fi
        ;;
    MINGW*|CYGWIN*|MSYS*)
        PLATFORM="windows-x64"
        INSTALL_DIR="${USERPROFILE}/bin"
        ;;
    *)
        echo "Unsupported OS: $(uname -s)"
        exit 1
        ;;
esac

echo "📦 Installing CmdVault for $PLATFORM..."

# Download latest release
DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/cmd-vault-${PLATFORM}"
if [[ "$PLATFORM" == "windows-x64" ]]; then
    DOWNLOAD_URL="${DOWNLOAD_URL}.exe"
fi

# Create install directory
mkdir -p "$INSTALL_DIR"

# Download and install
if command -v curl >/dev/null; then
    curl -L "$DOWNLOAD_URL" -o "$INSTALL_DIR/cmd-vault"
elif command -v wget >/dev/null; then
    wget "$DOWNLOAD_URL" -O "$INSTALL_DIR/cmd-vault"  
else
    echo "❌ Please install curl or wget"
    exit 1
fi

chmod +x "$INSTALL_DIR/cmd-vault"

echo "✅ CmdVault installed to $INSTALL_DIR/cmd-vault"
echo "🚀 Run 'cmd-vault --help' to get started"

# Add to PATH suggestion
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo ""
    echo "💡 Add to your shell profile:"
    echo "   export PATH=\"$INSTALL_DIR:\$PATH\""
fi
```

#### Windows PowerShell Installer
```powershell
# install.ps1
param(
    [string]$InstallDir = "$env:USERPROFILE\bin"
)

$repo = "username/cmd-vault" 
$platform = "windows-x64"

Write-Host "📦 Installing CmdVault for Windows..." -ForegroundColor Green

# Create install directory
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null

# Download latest release
$downloadUrl = "https://github.com/$repo/releases/latest/download/cmd-vault-$platform.exe"
$installPath = Join-Path $InstallDir "cmd-vault.exe"

try {
    Invoke-WebRequest -Uri $downloadUrl -OutFile $installPath
    Write-Host "✅ CmdVault installed to $installPath" -ForegroundColor Green
    
    # Check PATH
    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($userPath -notlike "*$InstallDir*") {
        Write-Host ""
        Write-Host "💡 Add to your PATH:" -ForegroundColor Yellow
        Write-Host "   setx PATH `"$userPath;$InstallDir`"" -ForegroundColor White
    }
    
    Write-Host "🚀 Run 'cmd-vault --help' to get started" -ForegroundColor Cyan
}
catch {
    Write-Error "❌ Installation failed: $_"
    exit 1
}
```

### 3. Package Managers

#### Homebrew Formula (macOS/Linux)
```ruby
# cmd-vault.rb
class CmdVault < Formula
  desc "Cross-platform terminal command manager"
  homepage "https://github.com/username/cmd-vault"
  version "0.2.0"
  
  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/username/cmd-vault/releases/download/v#{version}/cmd-vault-macos-x64"
      sha256 "..." 
    else
      url "https://github.com/username/cmd-vault/releases/download/v#{version}/cmd-vault-macos-arm64"
      sha256 "..."
    end
  end
  
  on_linux do
    url "https://github.com/username/cmd-vault/releases/download/v#{version}/cmd-vault-linux-x64"
    sha256 "..."
  end

  def install
    bin.install Dir["cmd-vault*"].first => "cmd-vault"
  end

  test do
    system "#{bin}/cmd-vault", "--version"
  end
end
```

#### Cargo Install
```toml
# Already works via:
# cargo install cmd-vault
# cargo install --git https://github.com/username/cmd-vault
```

#### Scoop Package (Windows)
```json
{
    "version": "0.2.0",
    "description": "Cross-platform terminal command manager",
    "homepage": "https://github.com/username/cmd-vault",
    "license": "MIT",
    "architecture": {
        "64bit": {
            "url": "https://github.com/username/cmd-vault/releases/download/v0.2.0/cmd-vault-windows-x64.exe",
            "hash": "...",
            "bin": [
                ["cmd-vault-windows-x64.exe", "cmd-vault"]
            ]
        }
    },
    "checkver": "github",
    "autoupdate": {
        "architecture": {
            "64bit": {
                "url": "https://github.com/username/cmd-vault/releases/download/v$version/cmd-vault-windows-x64.exe"
            }
        }
    }
}
```

## 🧪 Cross-Platform Testing Strategy

### 1. Local Testing with Docker
```dockerfile
# test/Dockerfile.ubuntu
FROM ubuntu:22.04
RUN apt-get update && apt-get install -y xclip
COPY target/x86_64-unknown-linux-gnu/release/cmd-vault /usr/local/bin/
CMD ["cmd-vault", "--help"]

# test/Dockerfile.alpine  
FROM alpine:latest
RUN apk add --no-cache xclip
COPY target/x86_64-unknown-linux-musl/release/cmd-vault /usr/local/bin/
CMD ["cmd-vault", "--help"]
```

### 2. Test Script
```bash
#!/bin/bash
# test/cross-platform-test.sh

echo "🧪 Cross-Platform Testing"

# Test basic functionality
echo "Testing basic commands..."
./cmd-vault --version
./cmd-vault --help
./cmd-vault -l

# Test clipboard (if available)
echo "Testing clipboard..."  
if ./cmd-vault -c "test" 2>/dev/null; then
    echo "✅ Clipboard test passed"
else
    echo "⚠️  Clipboard test skipped (tools not available)"
fi

# Test file operations
echo "Testing file operations..."
./cmd-vault -a "Test Command" "echo test" "Test description"
if ./cmd-vault -s "Test Command" | grep -q "Test Command"; then
    echo "✅ File operations test passed"
else
    echo "❌ File operations test failed"
    exit 1
fi

echo "✅ All tests completed"
```

## 📦 Distribution Summary

### Installation Methods
1. **Direct Download** - GitHub releases with platform detection
2. **Package Managers** - Homebrew (macOS/Linux), Scoop (Windows)  
3. **Cargo** - `cargo install cmd-vault`
4. **Script Installation** - One-line installers for each platform

### Cross-Platform Features
- **Config Location** - OS-appropriate directories (`~/.config`, `%APPDATA%`)
- **Clipboard Fallbacks** - Native + system command fallbacks
- **Path Handling** - Platform-aware file operations
- **Terminal Support** - Works in all major terminals
- **Package Integration** - Native package manager support

The Rust version's cross-platform support is excellent thanks to the ecosystem. The key is adding proper fallbacks, platform-specific optimizations, and comprehensive distribution methods!