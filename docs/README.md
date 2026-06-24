# 📚 CMD-VAULT Documentation

Welcome to the cmd-vault developer documentation! This directory contains all the guides, architecture docs, and resources you need to contribute effectively.

## 🚀 Getting Started

**New to the project?** Start here:

1. **[📖 Main README](../README.md)** - Project overview and basic setup
2. **[🚀 Quick Start Scripts](QUICK-START-SCRIPTS.md)** - Essential development commands
3. **[🏗️ Architecture Summary](ARCHITECTURE-SUMMARY.md)** - High-level system design
4. **[🦀 Rust Dev Workflow](RUST-DEV-WORKFLOW.md)** - Rust-specific development practices

## 📋 Documentation Index

### 🏗️ **Architecture & Design**
- **[Architecture Summary](ARCHITECTURE-SUMMARY.md)** - High-level system overview
- **[UI Architecture](UI-ARCHITECTURE.md)** - Terminal UI component structure
- **[Refactoring Roadmap](REFACTORING-ROADMAP.md)** - Code organization improvement plan

### 🛠️ **Development Guides**
- **[Rust Dev Workflow](RUST-DEV-WORKFLOW.md)** - Rust-specific practices and conventions
- **[UI Contribution Guide](UI-CONTRIBUTION-GUIDE.md)** - How to work with UI components
- **[Platform Dev Guide](PLATFORM-DEV-GUIDE.md)** - Cross-platform development
- **[Cross Platform Guide](cross-platform-guide.md)** - Platform-specific considerations
- **[Testing Guide](TESTING-GUIDE.md)** - Comprehensive testing practices
- **[DevOps Guide](DEVOPS-GUIDE.md)** - CI/CD, Docker, and deployment

### 🚀 **Tools & Scripts**
- **[Quick Start Scripts](QUICK-START-SCRIPTS.md)** - Essential development commands
- **[Scripts Documentation](../scripts/README.md)** - Detailed script usage guide

### 🔄 **CI/CD Management**
- **Check CI Status**: `./scripts/ci-status.sh` - See if CI is enabled/disabled
- **Enable CI**: `./scripts/ci-enable.sh` - Turn on GitHub Actions workflows  
- **Disable CI**: `./scripts/ci-disable.sh` - Turn off GitHub Actions workflows
- **CI Documentation**: [DevOps Guide](DEVOPS-GUIDE.md) - Complete CI/CD setup

## 📊 **Documentation by Role**

### 👩‍💻 **New Contributors**
Start with these docs in order:
1. [Quick Start Scripts](QUICK-START-SCRIPTS.md) - Get your environment running
2. [Architecture Summary](ARCHITECTURE-SUMMARY.md) - Understand the big picture
3. [Rust Dev Workflow](RUST-DEV-WORKFLOW.md) - Learn our conventions

### 🎨 **UI/UX Developers**  
Focus on these guides:
- [UI Architecture](UI-ARCHITECTURE.md) - Component structure
- [UI Contribution Guide](UI-CONTRIBUTION-GUIDE.md) - How to add/modify UI
- [Quick Start Scripts](QUICK-START-SCRIPTS.md) - Development tools

### 🔧 **Core/Backend Developers**
Essential reading:
- [Architecture Summary](ARCHITECTURE-SUMMARY.md) - System design
- [Rust Dev Workflow](RUST-DEV-WORKFLOW.md) - Code standards
- [Platform Dev Guide](PLATFORM-DEV-GUIDE.md) - Cross-platform code
- [Testing Guide](TESTING-GUIDE.md) - Testing best practices

### 🚀 **DevOps/Infrastructure**  
Pipeline and deployment:
- [DevOps Guide](DEVOPS-GUIDE.md) - CI/CD, Docker, deployment
- [Testing Guide](TESTING-GUIDE.md) - Automated testing
- [Architecture Summary](ARCHITECTURE-SUMMARY.md) - System overview

### 🏗️ **Maintainers/Architects**
Strategic documents:
- [Refactoring Roadmap](REFACTORING-ROADMAP.md) - Code organization goals
- [Architecture Summary](ARCHITECTURE-SUMMARY.md) - System overview
- All guides for comprehensive understanding

## 🛠️ **Quick Reference**

### Development Commands
```bash
# Code analysis
./scripts/code-stats.sh         # Visual file size analysis
./scripts/codebase-overview.sh  # Project structure guide

# Quality checks  
./scripts/dev-check.sh          # Full quality validation
cargo fmt                       # Format code
cargo clippy                    # Lint code
cargo test                      # Run tests

# Build & run
cargo run                       # Run in debug mode
cargo run --release             # Run optimized build
```

### File Structure
```
cmd-vault/
├── docs/                    # 📚 All documentation (you are here!)
├── src/                     # 🦀 Rust source code
├── scripts/                 # 🛠️ Development tools
├── README.md                # 📖 Main project overview
└── Cargo.toml              # 📦 Rust project configuration
```

## 🤝 **Contributing to Documentation**

Found outdated info or want to improve these docs?

1. **Small fixes**: Edit the relevant `.md` file directly
2. **New guides**: Add to `docs/` and update this index
3. **Major changes**: Discuss in issues/PRs first

**Documentation principles:**
- Keep guides focused and actionable
- Use clear examples and code snippets  
- Update related docs when making changes
- Test instructions before publishing

## 📞 **Need Help?**

- **Can't find what you need?** Check if we have an existing guide that covers it
- **Found a bug in the docs?** Please report it or submit a fix
- **Want a new guide?** Open an issue describing what would help

---

**Happy coding! 🦀✨**

*Last updated: When you moved docs to this folder*