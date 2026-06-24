# 🧪 Testing Guide

This guide explains the testing structure and practices for cmd-vault.

## 📁 Testing Structure

```
tests/
├── unit/                    # Unit tests (alongside source files)
├── integration/             # Integration tests
│   ├── cli_tests.rs        # CLI interface tests  
│   └── storage_tests.rs    # Storage layer tests
└── fixtures/               # Test data and fixtures
    └── sample_commands.json

benches/
└── search_performance.rs   # Performance benchmarks

examples/
└── basic_usage.rs         # Usage examples (also serve as tests)
```

## 🎯 Types of Tests

### Unit Tests
Located in `src/` files using `#[cfg(test)]` modules:
- Test individual functions and methods
- Fast execution (< 1ms per test)
- No external dependencies
- High code coverage target (>80%)

### Integration Tests  
Located in `tests/integration/`:
- Test complete workflows end-to-end
- CLI interface testing with `assert_cmd`
- File system interactions
- Cross-platform compatibility

### Performance Tests
Located in `benches/`:
- Benchmark search performance with different dataset sizes
- Memory usage profiling
- Regression detection

## 🚀 Running Tests

### All Tests
```bash
# Run complete test suite
cargo test

# Run with output
cargo test -- --nocapture

# Run tests in parallel (faster)
cargo test --release
```

### Specific Test Types
```bash
# Unit tests only
cargo test --lib

# Integration tests only  
cargo test --test '*'

# Specific integration test file
cargo test --test cli_tests

# Documentation tests
cargo test --doc
```

### Benchmarks
```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench search_performance

# Generate HTML reports
cargo bench -- --output-format html
```

### Coverage
```bash
# Install coverage tool
cargo install cargo-llvm-cov

# Generate coverage report
cargo llvm-cov --html

# Coverage with specific threshold
cargo llvm-cov --fail-under-lines 80
```

## 🎨 Test Patterns

### Test Fixtures
```rust
// Loading test data
fn load_test_commands() -> Vec<Command> {
    let fixture_path = "tests/fixtures/sample_commands.json";
    let contents = std::fs::read_to_string(fixture_path).unwrap();
    serde_json::from_str(&contents).unwrap()
}
```

### Temporary Files
```rust
use tempfile::tempdir;

#[test]
fn test_with_temp_file() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("test.json");
    
    // Use file_path for testing
    // Automatically cleaned up when temp_dir drops
}
```

### CLI Testing
```rust
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_command() {
    let mut cmd = Command::cargo_bin("cmd-vault").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("0.1.0"));
}
```

### Parameterized Tests
```rust
use rstest::rstest;

#[rstest]
#[case("docker", 1)]
#[case("git", 1)]  
#[case("nonexistent", 0)]
fn test_search_results(#[case] query: &str, #[case] expected_count: usize) {
    let storage = setup_test_storage();
    let results = storage.search(query);
    assert_eq!(results.len(), expected_count);
}
```

## 🛡️ Test Guidelines

### Writing Good Tests

**✅ Do:**
- Use descriptive test names: `test_search_returns_correct_results_for_docker_query`
- Test one thing per test
- Use `arrange-act-assert` pattern
- Clean up resources (use `tempfile` for files)
- Test edge cases and error conditions

**❌ Don't:**
- Write tests that depend on each other
- Test implementation details (test behavior, not internals)
- Use hardcoded paths or rely on specific filesystem state
- Ignore test failures or mark tests as `#[ignore]` without good reason

### Test Data Management
```rust
// Good: Use builder pattern for test data
impl Command {
    fn test_builder() -> CommandBuilder {
        CommandBuilder::new()
            .name("test-command")
            .command("echo test")
            .desc("Test command")
    }
}

// Usage
let cmd = Command::test_builder()
    .name("custom-name")
    .build();
```

### Error Testing
```rust
#[test]
fn test_duplicate_command_names() {
    let mut storage = Storage::new(temp_path());
    let cmd = test_command();
    
    // First add should succeed
    assert!(storage.add_command(cmd.clone()).is_ok());
    
    // Second add should fail with specific error
    match storage.add_command(cmd) {
        Err(StorageError::DuplicateName(_)) => {}, // Expected
        other => panic!("Expected DuplicateName error, got: {:?}", other),
    }
}
```

## 🔧 Test Configuration

### Cargo.toml Test Setup
```toml
[dev-dependencies]
assert_cmd = "2.0"      # CLI testing
predicates = "3.0"       # Assertion helpers  
tempfile = "3.8"         # Temporary files
criterion = "0.5"        # Benchmarking
pretty_assertions = "1.4" # Better assertion output
rstest = "0.18"          # Parameterized tests
```

### Test Environment Variables
```bash
# Disable output during tests
export RUST_LOG=off

# Use test-specific config location
export CMD_VAULT_CONFIG=/tmp/test-config.json

# Run tests with custom settings
RUST_BACKTRACE=1 cargo test
```

## 📊 CI/CD Integration

Tests are automatically run in CI for:
- Multiple Rust versions (stable, beta)
- Multiple platforms (Linux, macOS, Windows)  
- Coverage reporting
- Performance regression detection

See `.github/workflows/ci.yml` for complete CI configuration.

## 🎯 Coverage Goals

- **Unit tests**: >90% line coverage
- **Integration tests**: All CLI commands and workflows
- **Documentation**: All public APIs have doc tests
- **Performance**: No regression >10% without justification

## 🚀 Quick Commands

```bash
# Development test loop
cargo watch -x test

# Run tests with pretty output
cargo test --color=always 2>&1 | less -R

# Test specific module
cargo test storage::tests

# Run ignored tests
cargo test -- --ignored

# Run single-threaded (for debugging)
cargo test -- --test-threads=1
```