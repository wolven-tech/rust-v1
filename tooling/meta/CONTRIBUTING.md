# Contributing to Meta

Thank you for your interest in contributing to Meta! This document provides guidelines and instructions for contributing.

## Code of Conduct

Be respectful, professional, and constructive in all interactions.

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Git
- Make (optional but recommended)
- cargo-audit: `cargo install cargo-audit`
- pre-commit (optional): `pip install pre-commit`

### Setup Development Environment

```bash
# Clone the repository
git clone https://github.com/yourusername/rust-v1.git
cd rust-v1/tooling/meta

# Install dependencies
cargo build

# Run tests to verify setup
cargo test

# Install pre-commit hooks (optional)
pre-commit install
```

## Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/bug-description
```

### 2. Make Your Changes

- Write clear, concise code
- Follow Rust idioms and best practices
- Add tests for new functionality
- Update documentation as needed

### 3. Run Quality Gates

Before committing, ensure all quality gates pass:

```bash
# Navigate to meta directory
cd tooling/meta

# Run all checks individually using meta itself (dogfooding!)
meta run fmt          # Check formatting
meta run clippy       # Lint code
meta test             # Run tests
meta run audit        # Security audit
```

### 4. Commit Your Changes

```bash
# Stage your changes
git add .

# Commit with descriptive message
git commit -m "feat: add awesome feature"
# or
git commit -m "fix: resolve issue with XYZ"
```

#### Commit Message Format

We follow conventional commits:

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `style:` - Code style changes (formatting, etc.)
- `refactor:` - Code refactoring
- `test:` - Adding or updating tests
- `chore:` - Maintenance tasks

### 5. Push and Create PR

```bash
git push origin feature/your-feature-name
```

Then create a Pull Request on GitHub.

## Quality Standards

### Code Quality

All code must meet these standards:

#### 1. Formatting

```bash
# Check formatting
cd tooling/meta
meta run fmt

# Auto-format
meta run fmt-fix
```

Configuration: `rustfmt.toml`

#### 2. Linting

```bash
# Run clippy
cd tooling/meta
meta run clippy

# Auto-fix issues
meta run clippy-fix
```

Configuration: `.clippy.toml`

**Zero warnings policy:** All clippy warnings must be resolved.

#### 3. Testing

```bash
# Run all tests
cd tooling/meta
meta test

# Run specific test
cargo test test_name

# Run with coverage (not yet implemented in meta)
cargo tarpaulin --out Html
```

**Requirements:**
- All new features must have tests
- Maintain or improve code coverage
- Integration tests for user-facing features
- Unit tests for internal logic

#### 4. Security

```bash
# Run security audit
cd tooling/meta
meta run audit
```

**Policy:** No known security vulnerabilities allowed.

### Documentation

- Public APIs must have doc comments (`///`)
- Complex logic should have inline comments
- README.md should be updated for new features
- Examples should be added for new functionality

Example:

```rust
/// Spawns a process with the given arguments.
///
/// # Arguments
///
/// * `args` - Command-line arguments to pass to the process
/// * `working_dir` - Directory to run the command in
///
/// # Returns
///
/// Returns a `Result` containing the child process handle
///
/// # Examples
///
/// ```no_run
/// let child = adapter.spawn_in(&["run"], Path::new("apps/api"))?;
/// ```
pub fn spawn_in(&self, args: &[&str], working_dir: &Path) -> Result<tokio::process::Child>
```

## Testing Guidelines

### Unit Tests

Place unit tests in the same file as the code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Test implementation
    }
}
```

### Integration Tests

Place in `tests/` directory:

```rust
// tests/integration_tests.rs
use assert_cmd::Command;

#[test]
fn test_cli_command() {
    let mut cmd = Command::cargo_bin("meta").unwrap();
    cmd.arg("--version");
    cmd.assert().success();
}
```

### Test Coverage

We aim for >80% code coverage. Run coverage reports:

```bash
make coverage
# Opens HTML coverage report
```

## Architecture Guidelines

### Module Organization

```
src/
├── main.rs           # Entry point, minimal logic
├── cli.rs            # CLI parsing only
├── config.rs         # Configuration management
├── execution/        # Process execution logic
├── adapters/         # Tool-specific adapters
└── tui/              # Terminal UI components
```

### Design Principles

1. **Separation of Concerns**: Each module has a single, well-defined responsibility
2. **Error Handling**: Use `Result` and `anyhow` for error propagation
3. **Async**: Use `tokio` for async operations
4. **No Unwrap**: Prefer `?` operator or proper error handling
5. **Documentation**: All public APIs must be documented

### Adding a New Tool Adapter

To add support for a new tool:

1. Add tool configuration to `Config` struct in `config.rs`
2. Implement tool-specific logic in `adapters/mod.rs`
3. Add tests for the new adapter
4. Update documentation

Example:

```rust
// In spawn_in method
if self.command == "newtool" {
    cmd.arg("--headless");  // Tool-specific flags
    cmd.args(args);
} else {
    cmd.args(args);
}
```

## CI/CD Pipeline

### GitHub Actions

Our CI pipeline (`.github/workflows/meta-ci.yml`) runs:

1. **Tests** - Ubuntu, macOS, Windows (stable + beta Rust)
2. **Formatting** - `cargo fmt --check`
3. **Linting** - `cargo clippy -- -D warnings`
4. **Security Audit** - `cargo audit`
5. **Build** - Release builds for all platforms
6. **Coverage** - Code coverage report

### Local CI Simulation

Test all quality gates locally:

```bash
cd tooling/meta
meta run fmt && meta run clippy && meta test && meta run audit
```

This runs all checks that will be executed in CI.

## Pull Request Process

### Before Submitting

- [ ] All quality gates pass (`meta run fmt && meta run clippy && meta test && meta run audit`)
- [ ] Tests added for new features
- [ ] Documentation updated
- [ ] No warnings or errors
- [ ] Commit messages follow convention

### PR Description

Include:

1. **Summary** - Brief description of changes
2. **Motivation** - Why is this change needed?
3. **Changes** - List of specific changes made
4. **Testing** - How was this tested?
5. **Screenshots** - If UI changes (TUI screenshots welcome)

Example:

```markdown
## Summary
Adds support for Moon task runner

## Motivation
Many monorepos use Moon for task orchestration

## Changes
- Added Moon adapter in `adapters/mod.rs`
- Updated config to support `[tools.moon]`
- Added integration tests

## Testing
- Added unit tests for Moon adapter
- Tested with real Moon project
- All existing tests pass
```

### Review Process

1. Automated checks must pass
2. At least one maintainer approval required
3. Changes requested must be addressed
4. Squash commits before merge (optional)

## Common Issues

### Clippy Warnings

If you get clippy warnings:

```bash
# See detailed warnings
cargo clippy --all-targets --all-features

# Auto-fix some warnings
cargo clippy --fix --allow-dirty
```

### Test Failures

```bash
# Run specific failing test
cargo test test_name -- --nocapture

# Run with backtrace
RUST_BACKTRACE=1 cargo test
```

### Formatting Issues

```bash
# Auto-format all code
cargo fmt --all
```

## Getting Help

- 📧 Create an issue on GitHub
- 💬 Join discussions
- 📖 Read existing issues and PRs

## Recognition

Contributors will be:
- Listed in CONTRIBUTORS.md (if created)
- Mentioned in release notes
- Credited in commit history

Thank you for contributing to Meta! 🚀
