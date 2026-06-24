# 📦 CmdVault

A blazingly fast terminal command manager built in Rust. Store, search, and execute your favorite commands with an intuitive TUI and powerful CLI interface.

## ✨ Features

- **🖥️ Interactive TUI** - Beautiful terminal interface for browsing and managing commands
- **⚡ Lightning Search** - Instant fuzzy search across command names and content
- **📋 Smart Clipboard** - One-click copying with interactive variable substitution
- **🔄 Multiple Sort Modes** - Sort by name, date, or command length
- **🎯 CLI Interface** - Headless operations for scripts and automation
- **🔧 Variable Placeholders** - Interactive `<variable>` replacement system
- **💾 Auto-Save** - Commands persist automatically to `~/.cmd-vault.json`

## 🚀 Installation

```bash
# Clone the repository
git clone <repository-url>
cd cmd-vault

# Build the release binary
cargo build --release

# Optional: Install globally
cargo install --path .
```

## 📖 Usage

### TUI Mode (Interactive)

Launch the full terminal interface:

```bash
cmd-vault
```

#### 🎮 TUI Controls

| Key | Action |
|-----|--------|
| `↑/↓` or `j/k` | Navigate commands |
| `/` | Search commands |
| `Enter` | Expand command details |
| `y` | Copy command to clipboard |
| `a` | Add new command |
| `d` | Delete selected command |
| `s` | Sort commands |
| `q` or `Esc` | Quit |

#### 🔍 Search Mode
- Type to search command names and content
- Use `↑/↓` to navigate results
- Press `Enter` to expand or `Esc` to exit

#### ✏️ Adding Commands
1. Press `a` to start adding
2. Fill in **Name** → `Tab` → **Command** → `Tab` → **Description**
3. Press `Enter` to save

#### 🎯 Interactive Variables
Commands with `<placeholders>` like:
```bash
scp <local_file> <user>@<host>:<remote_path>
```

When copying, you'll get prompted to fill each variable:
```
Fill <local_file> (1/3): myfile.txt
Fill <user> (2/3): admin  
Fill <host> (3/3): server.com
```

Result: `scp myfile.txt admin@server.com:/path/`

### CLI Mode (Headless)

Perfect for scripts, automation, and quick terminal operations:

#### Search Commands
```bash
cmd-vault -s "ffmpeg"
cmd-vault -s "docker"
```

Output:
```
┌─ FFmpeg Compress Video
│  ffmpeg -i input.mp4 -vcodec libx265 -crf 28 output.mp4
│  Compresses mp4 video using the efficient H.265 codec to save space.
└─

  1 result(s) found
```

#### Copy First Match
```bash
cmd-vault -c "ffmpeg"
# Automatically copies the first matching command to clipboard
```

#### List All Commands
```bash
cmd-vault -l
```

Output:
```
📦 CmdVault (6 commands)

  1. FFmpeg Compress Video
     ffmpeg -i input.mp4 -vcodec libx265 -crf 28 output.mp4
     Compresses mp4 video using the efficient H.265 codec to save space.

  2. Docker Prune
     docker system prune -af  
     Remove all unused containers, images, and volumes
```

#### Add Command (Headless)
```bash
cmd-vault -a "My Command" "echo hello" "Test command"
```

### 🎛️ Sort Modes

Press `s` in TUI mode to access sort options:

1. **A → Z** (alphabetical)
2. **Z → A** (reverse alphabetical)  
3. **Newest first** (by creation date)
4. **Oldest first** (by creation date)
5. **Shortest command first** (by command length)

## 🏗️ Architecture

The project follows a clean modular architecture:

```
src/
├── main.rs          # Entry point and CLI argument handling
├── app.rs           # Core application state and business logic
├── cli.rs           # Command-line interface implementation  
├── handler.rs       # Event handling and input processing
├── ui.rs            # Terminal UI rendering and components
├── models.rs        # Data structures and serialization
├── storage.rs       # File I/O and persistence layer
├── placeholders.rs  # Interactive variable system
├── utils.rs         # Shared utilities and search logic
└── constants.rs     # Application constants and defaults
```

## 💾 Data Storage

Commands are stored in `~/.cmd-vault.json` as pretty-printed JSON:

```json
[
  {
    "name": "FFmpeg Compress Video",
    "command": "ffmpeg -i input.mp4 -vcodec libx265 -crf 28 output.mp4", 
    "desc": "Compresses mp4 video using the efficient H.265 codec to save space.",
    "created_at": 1703123456
  }
]
```

## 🔧 Development

### Build and Run
```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run with arguments
cargo run -- -s "docker"

# Run tests
cargo test
```

### Code Quality
```bash
# Check for issues
cargo clippy

# Format code
cargo fmt

# Security audit
cargo audit
```

## 🚨 Safety Features

- **Unique names enforced** - Prevents duplicate command entries
- **Input validation** - Sanitizes and validates all user input  
- **Safe deletions** - Confirmation prompts for destructive actions
- **Auto-backup** - Changes saved immediately to prevent data loss
- **Error handling** - Graceful failure with helpful error messages

## 🎨 UI Features

- **Syntax highlighting** in expanded view
- **Smart truncation** for long commands with ellipsis (…)
- **Responsive layout** adapts to terminal size
- **Status messages** provide clear feedback
- **Modal dialogs** for focused interactions
- **ANSI colors** and box-drawing characters in CLI output

## 📋 Examples

### Common Use Cases

**System Administration:**
```bash
# Add a system cleanup command
cmd-vault -a "System Cleanup" "sudo apt autoremove && sudo apt autoclean" "Clean up unused packages"

# Quick search and copy
cmd-vault -c "cleanup"
```

**Development Workflow:**
```bash
# Add a build command with variables
cmd-vault -a "Docker Build" "docker build -t <image_name>:<tag> ." "Build Docker image"

# Interactive variable filling in TUI mode
# When copying: Fill <image_name> → myapp, Fill <tag> → v1.0
# Result: docker build -t myapp:v1.0 .
```

**Network Operations:**
```bash
# Add SSH copy command
cmd-vault -a "SSH Copy" "scp <local_file> <user>@<host>:<remote_path>" "Copy files via SSH"

# Search and copy
cmd-vault -s "scp"
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Ratatui](https://github.com/ratatui-org/ratatui) for the terminal UI
- Uses [Crossterm](https://github.com/crossterm-rs/crossterm) for cross-platform terminal handling
- Powered by [Clap](https://github.com/clap-rs/clap) for CLI argument parsing
- Clipboard integration via [Copypasta](https://github.com/alacritty/copypasta)

---

**Made with ❤️ and 🦀 Rust**