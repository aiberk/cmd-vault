# 🦀 Rust Development Workflow & Quality Tools

## 🔧 Built-in Rust Tools

### **1. Type Checking (rustc)**
Rust's compiler provides **world-class type checking** by default:

```bash
# Type checking happens automatically during build
cargo check          # Fast type checking without building
cargo build          # Full compilation with type checking
cargo build --release # Optimized build with type checking
```

**What it catches:**
- Type mismatches
- Lifetime errors  
- Borrowing violations
- Memory safety issues
- Thread safety problems
- Pattern matching completeness

**Example:**
```rust
let x: i32 = "hello";  // ❌ Compiler error: expected i32, found &str
let mut vec = vec![1, 2, 3];
let first = &vec[0];
vec.push(4);          // ❌ Compiler error: cannot borrow as mutable
println!("{}", first); // while immutable borrow exists
```

### **2. Clippy (Rust's Linter) 🦀**
The official Rust linter - catches style issues and suggests improvements:

```bash
# Install (usually comes with Rust)
rustup component add clippy

# Run linting
cargo clippy                    # Basic linting
cargo clippy --all-targets     # Lint all code including tests
cargo clippy --all-features    # Lint with all features enabled
cargo clippy -- -D warnings    # Treat warnings as errors
```

**What Clippy catches:**
- Performance issues
- Idiomatic Rust style violations  
- Potential bugs
- Complexity issues
- Deprecated patterns

**Example output:**
```
warning: this loop could be written as a `for` loop
  --> src/main.rs:10:5
   |
10 |     while let Some(item) = iter.next() {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `for item in iter`
   |
   = note: `#[warn(clippy::while_let_on_iterator)]` on by default
```

### **3. Rustfmt (Code Formatting)**
Automatic code formatting - no more style debates:

```bash
# Install (usually comes with Rust)
rustup component add rustfmt

# Format code
cargo fmt                # Format all code in project
cargo fmt --check       # Check if code is formatted (CI)
```

**Configuration** (`rustfmt.toml`):
```toml
# Customize formatting rules
max_width = 100
tab_spaces = 4
newline_style = "Unix"
use_small_heuristics = "Default"
```

### **4. Testing Framework**
Built-in unit and integration testing:

```bash
# Run tests
cargo test              # Run all tests
cargo test --lib        # Run only library tests  
cargo test --bin        # Run only binary tests
cargo test test_name    # Run specific test
cargo test -- --nocapture  # Show println! output
```

**Example tests:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_functionality() {
        let items = vec![
            CommandItem { name: "test".to_string(), /* ... */ }
        ];
        let results = find_matching_items(&items, "test");
        assert_eq!(results.len(), 1);
    }

    #[test]
    #[should_panic(expected = "Invalid input")]
    fn test_invalid_input() {
        process_command("");  // Should panic
    }
}
```

## 🛠️ Advanced Development Tools

### **1. Cargo Audit (Security)**
Checks for security vulnerabilities in dependencies:

```bash
# Install
cargo install cargo-audit

# Check for vulnerabilities
cargo audit             # Scan dependencies
cargo audit fix         # Auto-fix known issues
```

### **2. Cargo Outdated (Dependency Management)**
Shows outdated dependencies:

```bash
# Install  
cargo install cargo-outdated

# Check outdated deps
cargo outdated          # List outdated dependencies
cargo outdated --root-deps-only  # Only direct dependencies
```

### **3. Cargo Bloat (Binary Size Analysis)**
Analyze what's making your binary large:

```bash
# Install
cargo install cargo-bloat

# Analyze binary size
cargo bloat --release          # What takes up space
cargo bloat --release --crates # By crate
```

### **4. Cargo Deny (License & Policy Checking)**
Enforce licensing and security policies:

```bash
# Install
cargo install cargo-deny

# Check policies (create deny.toml first)
cargo deny check        # Check all policies
cargo deny check licenses  # Only license compliance
```

**Example `deny.toml`:**
```toml
[licenses]
# Only allow these licenses
allow = ["MIT", "Apache-2.0", "BSD-3-Clause"]
deny = ["GPL-3.0"]  # Reject GPL licenses

[bans]
# Ban specific crates
deny = [
    { name = "openssl", reason = "Use rustls instead" }
]
```

## 📋 Pre-commit Setup

### **1. Create Pre-commit Hook**
```bash
# .git/hooks/pre-commit (make executable)
#!/bin/bash
set -e

echo "🔍 Running pre-commit checks..."

# Format check
echo "📝 Checking formatting..."
cargo fmt --check

# Linting
echo "🦀 Running Clippy..."  
cargo clippy --all-targets -- -D warnings

# Type checking
echo "🔧 Type checking..."
cargo check --all-targets

# Tests
echo "🧪 Running tests..."
cargo test --quiet

echo "✅ All checks passed!"
```

### **2. Alternative: Use pre-commit framework**
```bash
# Install pre-commit
pip install pre-commit

# Create .pre-commit-config.yaml
repos:
  - repo: local
    hooks:
      - id: cargo-fmt
        name: cargo fmt
        entry: cargo fmt
        language: system
        args: ["--", "--check"]
        files: \.rs$
      - id: cargo-clippy  
        name: cargo clippy
        entry: cargo clippy
        language: system
        args: ["--all-targets", "--", "-D", "warnings"]
        files: \.rs$

# Install hooks
pre-commit install
```

## 🏗️ CI/CD Configuration

### **GitHub Actions (.github/workflows/ci.yml)**
```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
      with:
        components: rustfmt, clippy
        
    - name: Cache dependencies
      uses: actions/cache@v3
      with:
        path: ~/.cargo
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
        
    - name: Format check
      run: cargo fmt --check
      
    - name: Clippy
      run: cargo clippy --all-targets -- -D warnings
      
    - name: Type check
      run: cargo check --all-targets
      
    - name: Run tests
      run: cargo test --verbose
      
    - name: Security audit
      run: |
        cargo install cargo-audit
        cargo audit

  coverage:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@stable
    
    - name: Install tarpaulin
      run: cargo install cargo-tarpaulin
      
    - name: Generate coverage
      run: cargo tarpaulin --verbose --all-features --workspace --timeout 120 --out xml
      
    - name: Upload to codecov.io
      uses: codecov/codecov-action@v3
```

## 🔧 IDE Integration

### **VS Code (Rust Analyzer)**
Best Rust development experience:

**Extensions:**
- `rust-analyzer` - Language server (intellisense, errors, refactoring)
- `CodeLLDB` - Debugging
- `crates` - Dependency management
- `Better TOML` - Cargo.toml support

**Settings (`.vscode/settings.json`):**
```json
{
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.cargo.buildScripts.enable": true,
    "rust-analyzer.procMacro.enable": true,
    "editor.formatOnSave": true,
    "editor.rulers": [100],
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust-analyzer"
    }
}
```

### **Other IDEs:**
- **IntelliJ IDEA**: Rust plugin (excellent but paid)
- **Vim/Neovim**: rust-analyzer + coc.nvim or built-in LSP
- **Emacs**: rust-analyzer + lsp-mode

## 🧪 Testing Strategies

### **1. Unit Tests**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_parsing() {
        let result = parse_command("echo 'hello world'");
        assert_eq!(result.binary, "echo");
        assert_eq!(result.args, vec!["hello world"]);
    }
}
```

### **2. Integration Tests**
```rust
// tests/integration_test.rs
use cmd_vault::storage;
use tempfile::tempdir;

#[test]
fn test_save_and_load_commands() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.json");
    
    let commands = vec![/* test data */];
    storage::save_items_to_file(&commands, &file_path).unwrap();
    
    let loaded = storage::load_items_from_file(&file_path).unwrap();
    assert_eq!(commands, loaded);
}
```

### **3. Property-Based Testing**
```rust
// Add to Cargo.toml: quickcheck = "1.0"
use quickcheck::quickcheck;

quickcheck! {
    fn test_search_never_panics(query: String, items: Vec<CommandItem>) -> bool {
        // Should never panic regardless of input
        let _ = search_items(&items, &query);
        true
    }
}
```

### **4. Benchmark Tests**
```rust
// benches/search_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cmd_vault::utils::find_matching_items;

fn search_benchmark(c: &mut Criterion) {
    let items = generate_test_items(1000);
    
    c.bench_function("search 1000 items", |b| {
        b.iter(|| {
            find_matching_items(black_box(&items), black_box("test"))
        })
    });
}

criterion_group!(benches, search_benchmark);
criterion_main!(benches);
```

## 📊 Code Quality Metrics

### **Project Structure for Quality:**
```
cmd-vault/
├── .github/workflows/        # CI/CD
├── benches/                 # Performance benchmarks  
├── src/                     # Source code
├── tests/                   # Integration tests
├── Cargo.toml              # Dependencies & metadata
├── rustfmt.toml            # Formatting rules
├── clippy.toml             # Linting configuration
└── deny.toml               # Security & license policies
```

### **Quality Commands:**
```bash
# Complete quality check
cargo fmt --check           # ✅ Formatting
cargo clippy -- -D warnings # ✅ Linting  
cargo test                  # ✅ Tests
cargo audit                 # ✅ Security
cargo outdated              # ✅ Dependencies
cargo bloat --release       # ✅ Binary size

# Advanced checks
cargo miri test             # ✅ Undefined behavior detection
cargo fuzz                  # ✅ Fuzzing for crash bugs
cargo tarpaulin             # ✅ Code coverage
```

## 🎯 Development Workflow

### **Daily Development:**
```bash
# 1. Start development
cargo check                 # Fast feedback loop

# 2. Write code with immediate feedback from rust-analyzer in IDE

# 3. Before committing
cargo fmt                   # Auto-format
cargo clippy --fix          # Auto-fix lint issues  
cargo test                  # Verify tests pass

# 4. Commit (pre-commit hooks run automatically)
```

### **PR Review Checklist:**
- ✅ `cargo fmt --check` passes
- ✅ `cargo clippy -- -D warnings` passes  
- ✅ `cargo test` passes
- ✅ New code has tests
- ✅ Documentation updated
- ✅ No new security vulnerabilities (`cargo audit`)

## 🏆 Rust's Advantages for Team Development

### **1. Compiler as Code Reviewer:**
- Catches 90% of bugs at compile time
- Enforces memory safety automatically
- Prevents data races and null pointer dereferences
- No undefined behavior (unlike C/C++)

### **2. Consistent Code Style:**
- `rustfmt` eliminates style debates
- Community conventions widely adopted
- Clippy enforces idiomatic patterns

### **3. Fearless Refactoring:**
- Type system catches breaking changes
- Refactoring tools work reliably
- Large codebases remain maintainable

### **4. Excellent Tooling:**
- Single `cargo` command for everything
- Integrated testing framework
- Built-in documentation generation (`cargo doc`)
- Package management with semantic versioning

This makes Rust **incredibly developer-friendly** for teams - the tooling prevents most common issues automatically, and developers get immediate feedback on code quality!