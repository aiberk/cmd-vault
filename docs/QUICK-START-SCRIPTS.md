# 🚀 Quick Start: Running Development Scripts

## ⚠️ IMPORTANT: Always run from project root!

```bash
# Make sure you're in the right place (where Cargo.toml is)
cd /Users/ai/Documents/Work/Shinerock/Products/TerminalTool-rust/cmd-vault

# Then run any script:
./scripts/SCRIPT-NAME.sh
```

## 📋 Essential Commands

### 📊 See Code Metrics (Visual Bar Graphs)
```bash
./scripts/code-stats.sh
```
Shows colorful bar graphs of file sizes, identifies large files to refactor.

### 🎯 Check Refactoring Progress  
```bash
./scripts/refactor-check.sh
```
Tracks progress toward code organization goals.

### 📖 Understand Project Structure
```bash
./scripts/codebase-overview.sh
```
Perfect for new contributors - shows what each file does.

### 🛠️ Setup Development Environment
```bash
./scripts/dev-setup.sh
```
First-time setup - checks tools, builds project.

### ✅ Run Quality Checks
```bash
./scripts/dev-check.sh
```
Comprehensive checks before committing code.

## 🐛 Troubleshooting

**"No such file or directory" errors?**
- Make sure you're in the project root (where `Cargo.toml` is)
- Don't run from inside the `scripts/` directory

**"Permission denied" errors?**
```bash
chmod +x scripts/*.sh
```

## 🎯 What to Expect

- **🔴 Red files** (400+ lines): Urgent refactoring needed
- **🟡 Yellow files** (200+ lines): Should be split soon  
- **🔵 Blue files** (100+ lines): Monitor growth
- **🟢 Green files** (<100 lines): Perfect size!

## 💡 Pro Tips

1. **Start new work**: `./scripts/codebase-overview.sh`
2. **Regular monitoring**: `./scripts/code-stats.sh` 
3. **Before commits**: `./scripts/dev-check.sh`
4. **Track progress**: `./scripts/refactor-check.sh`

---
**Happy coding! 🦀✨**