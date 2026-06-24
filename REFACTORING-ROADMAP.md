# 🔧 Refactoring Roadmap

Based on code analysis, here are the files that need attention and suggested improvements:

## 📊 Current Code Statistics

Run `./scripts/code-stats.sh` to see current line counts with visual bars.

## 🚨 Critical Refactoring Targets

### 1. `src/ui_old.rs` (607 lines) - **REMOVE**
- **Status**: 🔴 **URGENT - Dead Code**
- **Issue**: Not included in module tree, completely unused
- **Action**: Delete this file entirely
- **Impact**: -607 lines, cleaner codebase

### 2. `src/cli.rs` (414 lines) - **SPLIT**
- **Status**: 🔴 **URGENT - Too Large**
- **Current Structure**:
  - CLI argument parsing
  - Headless command execution
  - Individual command handlers (search, copy, list, add, etc.)
- **Suggested Split**:
  ```
  src/cli/
  ├── mod.rs          # Main CLI struct and run_headless()
  ├── parser.rs       # Argument parsing logic
  ├── commands/       # Individual command handlers
  │   ├── mod.rs
  │   ├── search.rs   # cmd_search()
  │   ├── copy.rs     # cmd_copy() 
  │   ├── list.rs     # cmd_list()
  │   ├── add.rs      # cmd_add()
  │   └── info.rs     # cmd_system_info(), cmd_manual()
  ```

### 3. `src/platform.rs` (398 lines) - **SPLIT BY PLATFORM**
- **Status**: 🟡 **HIGH - Logical Split Needed**
- **Current Structure**: All platform implementations in one file
- **Suggested Split**:
  ```
  src/platform/
  ├── mod.rs          # Public API and trait definitions
  ├── types.rs        # PlatformInfo, PlatformError, etc.
  ├── linux.rs        # LinuxPlatform implementation
  ├── macos.rs        # MacOSPlatform implementation
  └── windows.rs      # WindowsPlatform implementation
  ```

### 4. `src/app.rs` (359 lines) - **SPLIT BY CONCERN**
- **Status**: 🟡 **HIGH - State Management Split**
- **Suggested Split**:
  ```
  src/app/
  ├── mod.rs          # Main App struct
  ├── state.rs        # Core state management
  ├── input.rs        # Input handling and modes
  └── navigation.rs   # List navigation logic
  ```

### 5. `src/handler.rs` (327 lines) - **SPLIT BY EVENT TYPE**
- **Status**: 🟡 **HIGH - Event Handler Split**
- **Suggested Split**:
  ```
  src/handlers/
  ├── mod.rs          # Main event dispatcher
  ├── keyboard.rs     # Keyboard event handling
  ├── input.rs        # Text input handling
  └── commands.rs     # Command execution handling
  ```

## ✅ Well-Structured Files (Keep As-Is)

- `src/ui/modals.rs` (294 lines) - Good size, focused responsibility
- `src/ui/layout.rs` (194 lines) - Appropriate for layout logic
- `src/utils.rs` (168 lines) - Good utility collection
- `src/ui/components.rs` (120 lines) - Perfect component size
- All files under 100 lines - Well-scoped

## 🎯 Refactoring Priorities

### Phase 1: Quick Wins
1. **Delete `ui_old.rs`** - Immediate -607 lines
2. **Create `scripts/refactor-check.sh`** - Monitor progress

### Phase 2: Platform Split (Low Risk)
1. Split `platform.rs` by platform - clear boundaries
2. Each platform can be developed/tested independently

### Phase 3: CLI Modularization (Medium Risk)
1. Split CLI commands into separate files
2. Better testability and maintainability

### Phase 4: App/Handler Refactoring (Higher Risk)
1. Requires careful state management
2. More complex interdependencies

## 🛠️ Helpful Tools Created

- `scripts/code-stats.sh` - Visual line count analysis
- `scripts/refactor-check.sh` - Monitor refactoring progress
- This roadmap document

## 🎯 Success Metrics

- **Target**: Reduce average file size from current to ~250 lines (Rust-appropriate)
- **Goal**: No files over 500 lines (major refactoring threshold)
- **Ideal**: Most files under 400 lines (comfortable Rust module size)

## 📏 **Rust File Size Context**

**Why these numbers make sense for Rust:**
- Rust modules can be larger than other languages due to explicit error handling
- Match expressions and detailed type definitions add lines but improve safety
- 400-500 lines is common in production Rust codebases
- Focus should be on **cohesion** and **single responsibility** over raw line count

**Current files aren't actually problematic:**
- `cli.rs` (414 lines): Reasonable for a CLI module with multiple commands
- `platform.rs` (398 lines): Expected for cross-platform abstraction
- `app.rs` (359 lines): Normal size for main application state

## 🤝 Contributors Guide

1. **Before refactoring**: Run `./scripts/code-stats.sh` to see current state
2. **During refactoring**: Keep related functionality together
3. **After refactoring**: Run stats again to measure improvement
4. **Always**: Update this roadmap when completing phases

---

**Next Action**: Start with Phase 1 - delete `ui_old.rs` for immediate improvement!