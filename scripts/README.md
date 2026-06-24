# 🛠️ Development Scripts

This directory contains helpful scripts for developers working on cmd-vault.

## � **IMPORTANT: Run scripts from project root directory!**

```bash
# Make sure you're in the project root (where Cargo.toml is)
cd /path/to/cmd-vault

# Then run scripts like this:
./scripts/code-stats.sh
```

**❌ Don't run from scripts directory** - the scripts need to find the `src` folder!

## �📋 Available Scripts

### 🚀 Setup & Environment
chmod +x scripts/*.sh

#### `dev-setup.sh`
**Purpose**: First-time setup for new contributors
```bash
./scripts/dev-setup.sh
```
- Checks Rust/Cargo installation
- Installs development tools (clippy, rustfmt)
- Verifies project builds
- Shows quick start guide

### 📊 Code Analysis

#### `code-stats.sh` 
**Purpose**: Visual line count analysis with colored bar graphs
```bash
./scripts/code-stats.sh
```
- Shows line counts for all source files
- Color-coded visualization (🔴 urgent, 🟡 high, 🔵 medium, 🟢 good)
- Summary statistics and recommendations
- Identifies files over various size thresholds

#### `refactor-check.sh`
**Purpose**: Track refactoring progress against goals
```bash
./scripts/refactor-check.sh
```
- Monitors progress toward refactoring targets
- Shows quick wins available
- Calculates potential impact of changes
- Provides next action recommendations

#### `codebase-overview.sh`
**Purpose**: High-level project structure guide for new contributors
```bash
./scripts/codebase-overview.sh
```
- Shows module responsibilities
- Explains file organization
- Links to relevant documentation
- Provides contribution tips

### ✅ Quality Assurance

#### `dev-check.sh`
**Purpose**: Comprehensive quality checks before commits
```bash
./scripts/dev-check.sh
```
- Code formatting verification
- Clippy linting
- Type checking
- Test execution
- Security audit (if tools available)

## 🎯 Typical Workflows

### New Contributor Setup
```bash
# 1. First time setup
./scripts/dev-setup.sh

# 2. Understand the codebase
./scripts/codebase-overview.sh

# 3. See current code metrics
./scripts/code-stats.sh
```

### Regular Development
```bash
# Before starting work - check refactoring priorities
./scripts/refactor-check.sh

# During development - run quality checks
./scripts/dev-check.sh

# Monitor progress
./scripts/code-stats.sh
```

### Refactoring Sessions
```bash
# Check current state
./scripts/code-stats.sh
./scripts/refactor-check.sh

# ... do refactoring work ...

# Check progress
./scripts/refactor-check.sh
./scripts/code-stats.sh
```

## 🎨 Color Coding

The scripts use consistent color coding:
- 🔴 **Red**: Critical issues (400+ lines, urgent action needed)
- 🟡 **Yellow**: High priority (200+ lines, should address soon)  
- 🔵 **Blue**: Medium priority (100+ lines, monitor growth)
- 🟢 **Green**: Good size (<100 lines, well-scoped)

## 🎯 Refactoring Goals

Current targets (see `REFACTORING-ROADMAP.md` for details):
- **No files over 300 lines** 
- **Average file size under 150 lines**
- **80% of files under 200 lines**

## 💡 Tips

- **Run scripts from project root**: `./scripts/script-name.sh`
- **Check permissions**: Scripts should be executable (`chmod +x scripts/*.sh`)
- **View help**: Most scripts show usage when run without arguments
- **Regular monitoring**: Run `code-stats.sh` regularly to track code growth

## 🔧 Adding New Scripts

When adding new scripts:
1. Make them executable: `chmod +x scripts/new-script.sh`
2. Add clear header comment explaining purpose
3. Use consistent color coding and emoji
4. Update this README with the new script
5. Follow the existing naming convention

## 📚 Related Documentation

- `../REFACTORING-ROADMAP.md` - Detailed refactoring plan
- `../ARCHITECTURE-SUMMARY.md` - Project architecture  
- `../UI-CONTRIBUTION-GUIDE.md` - UI development guide
- `../RUST-DEV-WORKFLOW.md` - Rust-specific practices