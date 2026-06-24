# 🚀 DevOps & CI/CD Guide

Professional DevOps setup for cmd-vault with automated testing, building, and deployment.

## 📁 DevOps Structure

```
.github/
├── workflows/
│   ├── ci.yml              # Continuous Integration
│   └── release.yml         # Automated releases
└── ISSUE_TEMPLATE/         # GitHub issue templates

ci/
├── install-deps.sh         # Dependency installation
└── test.sh                 # Comprehensive test runner

docker/
├── Dockerfile              # Production container
├── Dockerfile.dev          # Development container  
└── docker-compose.yml      # Container orchestration
```

## 🔄 CI/CD Pipeline

### Continuous Integration (`.github/workflows/ci.yml`)

**Triggers:**
- Push to `main` or `develop` branches
- Pull requests to `main` or `develop`

**Jobs:**
1. **Check** - Code quality and formatting
2. **Test** - Multi-platform test suite
3. **Security** - Security audit
4. **Coverage** - Code coverage reporting
5. **Benchmark** - Performance regression detection
6. **Build** - Cross-platform binary builds

**Matrix Testing:**
- **OS**: Ubuntu, Windows, macOS
- **Rust**: stable, beta
- **Features**: All combinations

### Release Pipeline (`.github/workflows/release.yml`)

**Triggers:**
- Git tags matching `v*` pattern (e.g., `v1.0.0`)

**Automated Steps:**
1. Build release binaries for all platforms
2. Create GitHub release
3. Upload platform-specific binaries
4. Publish to crates.io (if configured)

## 🐳 Docker Setup

### Production Container
```dockerfile
# Multi-stage build for minimal image size
FROM rust:1.75-slim as builder
# ... build steps
FROM debian:bookworm-slim
# ... runtime setup
```

**Features:**
- Multi-stage build for minimal image size
- Non-root user for security
- SSL certificates included
- Optimized for production use

### Development Container
```dockerfile  
FROM rust:1.75
# Development tools and hot reloading
```

**Features:**
- Hot reloading with `cargo-watch`
- Volume mounting for source code
- Development tool pre-installed

### Usage
```bash
# Build production image
docker build -t cmd-vault .

# Run production container
docker run -it cmd-vault

# Development with hot reload
docker-compose up cmd-vault-dev

# Run tests in container
docker-compose run cmd-vault-dev cargo test
```

## 🛡️ Security & Quality

### Security Scanning
```bash
# Dependency vulnerability scan
cargo audit

# License and security policy check  
cargo deny check

# OWASP dependency check (if applicable)
```

### Code Quality Gates
- **Formatting**: `cargo fmt --check`
- **Linting**: `cargo clippy -- -D warnings`
- **Tests**: All tests must pass
- **Coverage**: Minimum 80% line coverage
- **Security**: No known vulnerabilities

### Performance Monitoring
- Benchmark regression detection
- Binary size tracking
- Build time optimization

## 📊 Monitoring & Metrics

### GitHub Actions Metrics
- Build success rate
- Test execution time
- Binary sizes across platforms
- Coverage trends

### Cargo Metrics
```toml
# In Cargo.toml
[profile.release]
lto = true              # Link-time optimization
codegen-units = 1       # Single codegen unit for smaller binaries
strip = true            # Strip debug symbols
```

## 🚀 Deployment Strategies

### Binary Releases
- **GitHub Releases**: Automated binary uploads
- **Crates.io**: Rust package registry
- **Homebrew**: macOS package manager (future)
- **Chocolatey**: Windows package manager (future)

### Container Deployment
```yaml
# Example Kubernetes deployment
apiVersion: apps/v1
kind: Deployment
metadata:
  name: cmd-vault
spec:
  replicas: 1
  selector:
    matchLabels:
      app: cmd-vault
  template:
    metadata:
      labels:
        app: cmd-vault
    spec:
      containers:
      - name: cmd-vault
        image: cmd-vault:latest
        resources:
          requests:
            memory: "64Mi"
            cpu: "50m"
          limits:
            memory: "128Mi"
            cpu: "100m"
```

## 🔧 Development Workflow

### Local Development
```bash
# Install development dependencies
./ci/install-deps.sh

# Run full test suite
./ci/test.sh

# Development loop with watch
cargo watch -x test -x run
```

### Pre-commit Hooks
```bash
# Install pre-commit hooks
cat > .git/hooks/pre-commit << 'EOF'
#!/bin/sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
EOF
chmod +x .git/hooks/pre-commit
```

### Branch Protection Rules
Configure in GitHub repository settings:
- Require pull request reviews
- Require status checks to pass
- Require up-to-date branches
- Include administrators

## 📈 Performance Optimization

### Build Optimization
```toml
# Fast development builds
[profile.dev]
debug = true
opt-level = 0

# Optimized test builds  
[profile.test]
debug = true
opt-level = 1

# Maximum optimization for release
[profile.release]
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

### CI/CD Optimization
- **Caching**: Cargo registry and target directories
- **Parallel execution**: Matrix builds across platforms
- **Incremental builds**: Only rebuild changed components
- **Artifact reuse**: Share binaries between jobs

## 🔍 Debugging CI Issues

### Common Issues and Solutions

**Build Failures:**
```bash
# Check Rust version compatibility
rustup show

# Clear cargo cache
cargo clean

# Verbose build output
cargo build --verbose
```

**Test Failures:**
```bash
# Run specific failing test
cargo test --test integration_tests -- --nocapture

# Check test environment
env | grep -i test
```

**Platform-specific Issues:**
- Check platform-specific dependencies
- Verify cross-compilation setup
- Test locally with Docker containers

## 📋 Checklists

### Pre-Release Checklist
- [ ] All tests passing
- [ ] Version updated in `Cargo.toml`
- [ ] Changelog updated
- [ ] Documentation updated
- [ ] Security audit clean
- [ ] Performance benchmarks acceptable

### CI/CD Health Check
- [ ] All workflows running successfully
- [ ] Coverage reports generating
- [ ] Security scans clean
- [ ] Build artifacts uploading
- [ ] Release process tested

## 🛠️ Tools and Services

### Required Tools
- **GitHub Actions**: CI/CD platform
- **Cargo**: Rust package manager
- **Docker**: Containerization
- **cargo-audit**: Security scanning
- **cargo-llvm-cov**: Code coverage

### Optional Tools
- **Codecov**: Coverage reporting service
- **Dependabot**: Automated dependency updates
- **cargo-deny**: License and security policy enforcement
- **cargo-outdated**: Dependency update checking

## 📞 Troubleshooting

### CI Pipeline Issues
1. Check GitHub Actions logs
2. Verify environment variables
3. Test locally with `act` (GitHub Actions runner)
4. Check platform-specific issues

### Docker Issues  
1. Verify Dockerfile syntax
2. Check base image availability
3. Test multi-stage build locally
4. Verify dependencies for target architecture

### Release Issues
1. Check tag format (must match `v*`)
2. Verify GitHub token permissions
3. Check crates.io token configuration
4. Verify binary signing (if applicable)

---

This DevOps setup provides a solid foundation for professional Rust project development with automated testing, security scanning, and deployment capabilities.