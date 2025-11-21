# Rust Zero Dead Code Skill

## Purpose
This skill enforces strict zero dead code policy, Clean Architecture patterns, SOLID principles, and Rust best practices across all Rust projects in the monorepo. Every line of code must serve a purpose.

## When to Use
Activate this skill when:
- Writing Rust code in any service (API V1, Meta CLI, etc.)
- Implementing domain logic, services, repositories, or handlers
- Working with async/await, SQLx, or CLI tools
- Refactoring existing Rust code
- Reviewing code for quality gates

---

## 🚫 ZERO DEAD CODE POLICY (ABSOLUTE)

### Rule: Every item must be used or removed

**NO exceptions. NO `#[allow(dead_code)]` in production code.**

```rust
// ❌ ABSOLUTELY FORBIDDEN
#[allow(dead_code)]
pub fn unused_function() { }

// ❌ ABSOLUTELY FORBIDDEN
pub struct Tool {
    used_field: String,
    #[allow(dead_code)]
    unused_field: String,  // Remove this!
}

// ✅ CORRECT - Only keep what's used
pub struct Tool {
    used_field: String,
}
```

### How to Handle "Future Use" Code

```rust
// ❌ WRONG - Keeping unused methods for "future use"
impl ToolAdapter {
    pub fn execute(&self) { }  // Not used anywhere
    pub fn spawn(&self) { }    // Not used anywhere
    pub fn execute_in(&self) { }  // Actually used
}

// ✅ CORRECT - Only keep what's used NOW
impl ToolAdapter {
    pub fn execute_in(&self) { }  // Keep only this
}

// If you need the other methods later, they're in git history!
```

### The ONLY Allowed Exception: Integration Tests

```rust
// ✅ ACCEPTABLE - Test-only code with clear documentation
#[cfg(test)]
impl Account {
    /// Test helper - not used in production
    pub fn new_for_test() -> Self {
        Self {
            id: "test-123".to_string(),
            balance: dec!(1000.00),
        }
    }
}

// ❌ WRONG - Dead code in production build
impl Account {
    #[allow(dead_code)]  // NO!
    pub fn new_for_test() -> Self { ... }
}
```

---

## 🏛️ Clean Architecture Layers (ENFORCED)

### Dependency Rule: Dependencies Point INWARD ONLY

```
Presentation → Application → Domain ← Infrastructure
   (HTTP/CLI)     (Services)   (Entities)   (Database, External)
```

### Layer Responsibilities

#### 1. **Domain Layer** (Core Business Logic)
- **Contains**: Entities, value objects, domain errors, repository traits
- **Dependencies**: NONE (pure Rust, no external crates except basic ones like serde, thiserror)
- **Location**: `src/domain/` or `domain/`
- **Dead Code**: NONE allowed

```rust
// ✅ CORRECT - Domain entity (all methods used)
#[derive(Debug, Clone)]
pub struct Project {
    name: String,
    path: String,
    project_type: ProjectType,
}

impl Project {
    // Only include methods that are actually called
    pub fn new(name: String, path: String, project_type: ProjectType) -> Self {
        Self { name, path, project_type }
    }

    // If this is never called, DELETE IT
    pub fn is_rust(&self) -> bool {
        matches!(self.project_type, ProjectType::Rust)
    }
}

// ❌ WRONG - Unused methods
impl Project {
    pub fn new(name: String) -> Self { ... }  // Used

    #[allow(dead_code)]
    pub fn from_path(path: &Path) -> Self { ... }  // NOT USED - DELETE!

    #[allow(dead_code)]
    pub fn validate(&self) -> Result<()> { ... }  // NOT USED - DELETE!
}
```

#### 2. **Application Layer** (Use Cases & Orchestration)
- **Contains**: Services that orchestrate domain objects
- **Dependencies**: Domain layer only
- **Location**: `src/application/` or `execution/`
- **Dead Code**: NONE allowed

```rust
// ✅ CORRECT - Only methods that are called
pub struct TaskExecutor {
    config: Config,
    adapters: HashMap<String, ToolAdapter>,
}

impl TaskExecutor {
    pub fn new(config: Config) -> Self { ... }  // Used in main.rs

    pub async fn execute_task(&self, task: &str) -> Result<()> { ... }  // Used in handlers
}

// ❌ WRONG - Keeping methods "just in case"
impl TaskExecutor {
    pub fn new(config: Config) -> Self { ... }  // Used

    #[allow(dead_code)]
    pub fn execute(&self) -> Result<()> { ... }  // NOT USED - DELETE!

    #[allow(dead_code)]
    pub fn spawn(&self) -> Result<()> { ... }  // NOT USED - DELETE!
}
```

#### 3. **Infrastructure Layer** (External Concerns)
- **Contains**: Database implementations, file I/O, external service adapters
- **Dependencies**: Domain layer (implements traits)
- **Location**: `src/infrastructure/` or `adapters/`
- **Dead Code**: NONE allowed

```rust
// ✅ CORRECT - Adapter with only used methods
pub struct ToolAdapter {
    name: String,
    command: String,
}

impl ToolAdapter {
    pub fn new(name: String, command: String) -> Self {
        Self { name, command }
    }

    // Only keep the variants that are actually called
    pub async fn execute_in(&self, args: &[&str], dir: &Path) -> Result<()> {
        // Implementation
        Ok(())
    }

    pub fn spawn_in(&self, args: &[&str], dir: &Path) -> Result<Child> {
        // Implementation
        Ok(child)
    }
}

// ❌ WRONG - Unused method variants
impl ToolAdapter {
    pub fn new(name: String, command: String) -> Self { ... }  // Used

    #[allow(dead_code)]
    pub fn execute(&self, args: &[&str]) -> Result<()> {  // NOT USED - DELETE!
        // "Future use" is not a valid reason
        Ok(())
    }

    #[allow(dead_code)]
    pub fn spawn(&self, args: &[&str]) -> Result<Child> {  // NOT USED - DELETE!
        // Git history exists for a reason
        Ok(child)
    }
}
```

#### 4. **Presentation Layer** (HTTP/CLI Handlers)
- **Contains**: CLI parsers, HTTP handlers, DTOs, request/response mapping
- **Dependencies**: Application and Domain layers
- **Location**: `src/presentation/`, `src/cli.rs`, `src/tui/`
- **Dead Code**: NONE allowed

```rust
// ✅ CORRECT - Only defined commands that are handled
#[derive(Subcommand)]
pub enum Commands {
    Init,
    Dev { projects: Option<Vec<String>> },
    Build { prod: bool, projects: Option<Vec<String>> },
    Test { watch: bool },
    Run { task: String, projects: Option<Vec<String>> },
    Tui,
}

// ❌ WRONG - Commands that aren't implemented
#[derive(Subcommand)]
pub enum Commands {
    Init,
    Dev { projects: Option<Vec<String>> },

    #[allow(dead_code)]
    Deploy { env: String },  // NOT IMPLEMENTED - DELETE!

    #[allow(dead_code)]
    Rollback { version: String },  // NOT IMPLEMENTED - DELETE!
}
```

---

## 🦀 Rust-Specific Best Practices

### Error Handling (MANDATORY)

#### Layer-Specific Error Types

```rust
// ✅ CORRECT - Only error variants that are actually used
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Configuration file not found: {0}")]
    NotFound(String),

    #[error("Invalid TOML: {0}")]
    InvalidToml(#[from] toml::de::Error),
}

// ❌ WRONG - Error variants that are never constructed
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Configuration file not found: {0}")]
    NotFound(String),  // Used

    #[error("Invalid TOML: {0}")]
    InvalidToml(#[from] toml::de::Error),  // Used

    #[allow(dead_code)]
    #[error("Permission denied: {0}")]
    PermissionDenied(String),  // NEVER USED - DELETE!

    #[allow(dead_code)]
    #[error("Network error: {0}")]
    Network(String),  // NEVER USED - DELETE!
}
```

#### NEVER use unwrap() in production

```rust
// ❌ WRONG - Will panic!
let value = option.unwrap();

// ✅ CORRECT - Proper error handling
let value = option.ok_or(Error::ValueNotFound)?;
```

### Async/Await Patterns

```rust
// ✅ CORRECT - Async trait with async_trait (when used)
#[async_trait]
pub trait Repository: Send + Sync {
    async fn find_by_id(&self, id: &str) -> Result<Option<Entity>>;
}

// ❌ WRONG - Async trait that's never implemented
#[allow(dead_code)]
#[async_trait]
pub trait UnusedRepository: Send + Sync {  // DELETE THIS!
    async fn method(&self) -> Result<()>;
}
```

### Type Safety with Newtypes

```rust
// ✅ CORRECT - Newtype that's actually used
#[derive(Debug, Clone)]
pub struct ProjectName(String);

impl ProjectName {
    pub fn new(name: String) -> Result<Self, ValidationError> {
        if name.is_empty() {
            return Err(ValidationError::EmptyName);
        }
        Ok(Self(name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ❌ WRONG - Newtype that's defined but never used
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TaskId(String);  // DELETE IF NOT USED!

impl TaskId {
    #[allow(dead_code)]
    pub fn new(id: String) -> Self {
        Self(id)
    }
}
```

---

## 🚫 Rust Anti-Patterns (FORBIDDEN)

### ❌ NEVER DO:

1. **Allow dead code with attributes**
```rust
// ❌ ABSOLUTELY FORBIDDEN
#[allow(dead_code)]
pub fn helper() { }

#[allow(dead_code)]
pub struct Unused { }

// ✅ CORRECT - Delete unused code
// (nothing here, it's been removed!)
```

2. **Keep code "for future use"**
```rust
// ❌ WRONG - "Future use" justification
impl Tool {
    pub fn execute_in(&self, dir: &Path) -> Result<()> { ... }  // Used

    /// Part of public API, kept for future use
    #[allow(dead_code)]
    pub fn execute(&self) -> Result<()> { ... }  // DELETE THIS!
}

// ✅ CORRECT - Only what's used now
impl Tool {
    pub fn execute_in(&self, dir: &Path) -> Result<()> { ... }
}

// If you need execute() later:
// 1. Check git history
// 2. Restore it
// 3. Use it immediately
```

3. **Use `unwrap()` or `expect()` in production**
```rust
// ❌ WRONG - Will panic!
let value = some_option.unwrap();

// ✅ CORRECT - Handle errors
let value = some_option.ok_or(Error::ValueNotFound)?;
```

4. **Put business logic in CLI handlers**
```rust
// ❌ WRONG - Business logic in CLI handler
match cli.command {
    Commands::Dev { projects } => {
        // Validation and orchestration logic here
        if projects.is_empty() {
            return Err("No projects specified".into());
        }
        // Complex business logic...
    }
}

// ✅ CORRECT - Delegate to service
match cli.command {
    Commands::Dev { projects } => {
        let config = Config::load()?;
        execution::dev(&config, projects).await?;
    }
}
```

5. **Suppress compiler warnings**
```rust
// ❌ ABSOLUTELY FORBIDDEN
#![allow(warnings)]
#![allow(dead_code)]

// ✅ CORRECT - Deny warnings
#![deny(warnings)]
#![deny(dead_code)]
```

---

## 🧪 Testing Requirements

### Test-Only Code (The ONLY Exception)

```rust
// ✅ ACCEPTABLE - Clearly marked as test code
#[cfg(test)]
mod tests {
    use super::*;

    impl Config {
        /// Test helper - creates minimal config for testing
        pub fn new_for_test() -> Self {
            Self {
                projects: HashMap::new(),
                tools: HashMap::new(),
            }
        }
    }

    #[test]
    fn test_config_validation() {
        let config = Config::new_for_test();
        assert!(config.projects.is_empty());
    }
}

// ❌ WRONG - Test code in production
impl Config {
    #[allow(dead_code)]
    pub fn new_for_test() -> Self { ... }  // DELETE! Use #[cfg(test)]
}
```

---

## 📋 Code Review Checklist

Before committing Rust code:

- [ ] **NO `#[allow(dead_code)]` anywhere** (except `#[cfg(test)]` blocks)
- [ ] Run `cargo build` - compiler will catch dead code
- [ ] Run `cargo clippy` - ZERO warnings allowed
- [ ] Run `cargo fmt` - code properly formatted
- [ ] All tests passing (`cargo test`)
- [ ] Every function/method is actually called
- [ ] Every struct field is actually used
- [ ] Every error variant is actually constructed
- [ ] Every trait is actually implemented
- [ ] Git history exists - don't keep code "just in case"

---

## 🔍 How to Find and Remove Dead Code

### Step 1: Build with warnings as errors

```bash
# Add to Cargo.toml or as rustflags
RUSTFLAGS="-D warnings -D dead_code" cargo build
```

### Step 2: Compiler will report ALL dead code

```
warning: method `execute` is never used
  --> src/adapters/mod.rs:18:18
   |
18 |     pub async fn execute(&self, args: &[&str]) -> Result<()> {
   |                  ^^^^^^^

warning: method `spawn` is never used
  --> src/adapters/mod.rs:70:12
   |
70 |     pub fn spawn(&self, args: &[&str]) -> Result<Child> {
   |            ^^^^^
```

### Step 3: DELETE the unused code

```rust
// Before (with dead code)
impl ToolAdapter {
    pub async fn execute(&self, args: &[&str]) -> Result<()> { ... }  // UNUSED
    pub async fn execute_in(&self, args: &[&str], dir: &Path) -> Result<()> { ... }  // USED
    pub fn spawn(&self, args: &[&str]) -> Result<Child> { ... }  // UNUSED
    pub fn spawn_in(&self, args: &[&str], dir: &Path) -> Result<Child> { ... }  // USED
}

// After (dead code removed)
impl ToolAdapter {
    pub async fn execute_in(&self, args: &[&str], dir: &Path) -> Result<()> { ... }
    pub fn spawn_in(&self, args: &[&str], dir: &Path) -> Result<Child> { ... }
}
```

### Step 4: Verify no warnings

```bash
cargo clippy -- -D warnings -D dead_code
# Should output: Finished with 0 warnings
```

---

## 💡 Philosophy

**"Code is a liability, not an asset."**

- Every line of code must be maintained
- Every line of code can contain bugs
- Every line of code makes the codebase harder to understand
- Git history preserves deleted code perfectly
- **YAGNI (You Aren't Gonna Need It)** - Don't write code for hypothetical future needs

**If it's not used NOW, delete it NOW.**

---

## Quick Reference

### Zero Dead Code Enforcement:

```toml
# .clippy.toml
# No configuration needed - deny dead_code by default!

# Cargo.toml
[lints.rust]
dead_code = "deny"
unused_imports = "deny"
unused_variables = "deny"
```

### Quality Gates:

```bash
# All must pass with ZERO warnings
cargo fmt --check
cargo clippy -- -D warnings -D dead_code
cargo test
```

---

**Remember**:
- Dead code is DELETED code
- Git history is your safety net
- YAGNI: You Aren't Gonna Need It
- Less code = Less bugs = Easier maintenance
