# Testing Patterns

**Analysis Date:** 2026-01-29

## Test Framework

**Runner:**
- Built-in Rust test runner (`cargo test`)
- No external test framework (no jest, vitest equivalent)
- Tests use `#[cfg(test)]` modules in source files

**Assertion Library:**
- Standard Rust `assert!`, `assert_eq!`, `assert_ne!` macros
- `anyhow::Result` for test functions that can fail

**Run Commands:**
```bash
# Run all tests
cargo test

# Test specific crate
cargo test --package sqlez
cargo test --package db

# Test specific module
cargo test --package sqlez savepoint::tests

# Run with output
cargo test -- --nocapture

# Run ignored tests
cargo test -- --ignored
```

## Test File Organization

**Location:**
- Tests are co-located with source code using `#[cfg(test)]` modules
- No separate `tests/` directories for unit tests
- Integration tests would go in `tests/` directory at crate root (not currently used)

**Naming:**
- Test modules named `tests` (e.g., `#[cfg(test)] mod tests`)
- Test functions use `snake_case` with descriptive names
- Test functions prefixed with `test_` (conventional, not required)

**Structure:**
```rust
// At end of source file
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_specific_behavior() {
        // Test implementation
    }
}
```

## Test Structure

**Suite Organization:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_basic_functionality() -> Result<()> {
        // Setup
        let connection = Connection::open_memory(Some("test_db"));

        // Execute
        let result = connection.some_operation()?;

        // Assert
        assert_eq!(result, expected_value);
        Ok(())
    }

    #[test]
    fn test_error_case() {
        let result = fallible_operation();
        assert!(result.is_err());
    }
}
```

**Patterns:**
- Return `anyhow::Result<()>` from tests to use `?` operator
- Use in-memory databases for database tests
- Clean up resources via Drop impls (no explicit teardown needed)

## Mocking

**Approach:**
- No mocking framework detected
- Manual mock implementations using trait objects
- Mock repositories provide sample data for development/testing

**Example from `crates/assets_ui/src/repository/mock_repository.rs`:**
```rust
pub struct MockAssetRepository {
    assets: Vec<AssetNode>,
}

impl MockAssetRepository {
    pub fn new() -> Self {
        Self {
            assets: Self::generate_sample_assets(),
        }
    }
}

impl AssetRepository for MockAssetRepository {
    fn get_all_assets(&self) -> Vec<AssetNode> {
        self.assets.clone()
    }
    // ...
}
```

**What to Mock:**
- Database connections (use in-memory SQLite)
- External services (HTTP clients, file system)
- Time-dependent operations

**What NOT to Mock:**
- Internal data structures
- Pure functions
- GPUI components (test at higher level)

## Fixtures and Factories

**Test Data:**
- Sample data generated via factory methods
- In-memory databases with known state
- No external fixture files detected

**Example pattern:**
```rust
fn generate_sample_assets() -> Vec<AssetNode> {
    vec![
        AssetNode {
            id: "test-1".to_string(),
            name: "Test Asset".to_string(),
            // ...
        },
    ]
}
```

**Location:**
- Factory methods in mock implementations
- Test data generators in test modules

## Database Testing

**Test Database Setup:**
```rust
#[cfg(any(test, feature = "test-support"))]
pub async fn open_test_db<M: Migrator>(db_name: &str) -> ThreadSafeConnection {
    ThreadSafeConnection::builder::<M>(db_name, false)
        .with_db_initialization_query(DB_INITIALIZE_QUERY)
        .with_connection_initialize_query(CONNECTION_INITIALIZE_QUERY)
        .with_write_queue_constructor(locking_queue())
        .build()
        .await
        .unwrap()
}
```

**Usage in tests:**
```rust
#[test]
fn test_kvp() {
    smol::block_on(async {
        let db = KeyValueStore::open_test_db("test_kvp").await;

        assert_eq!(db.read_kvp("key-1").unwrap(), None);

        db.write_kvp("key-1".to_string(), "one".to_string())
            .await
            .unwrap();
        assert_eq!(db.read_kvp("key-1").unwrap(), Some("one".to_string()));
    });
}
```

## Coverage

**Requirements:**
- No explicit coverage target enforced
- No coverage tool configured (no tarpaulin, cargo-llvm-cov detected)

**View Coverage:**
```bash
# Install cargo-tarpaulin for coverage
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html
```

## Test Types

**Unit Tests:**
- Located in `#[cfg(test)]` modules
- Test single functions/methods in isolation
- Use in-memory dependencies

**Integration Tests:**
- Not currently used (no `tests/` directories)
- Would test crate public API

**E2E Tests:**
- Not detected
- Would require GPUI test harness

## Common Patterns

**Async Testing:**
```rust
#[test]
fn test_async_operation() {
    smol::block_on(async {
        let result = async_operation().await;
        assert!(result.is_ok());
    });
}
```

**Database Transaction Testing:**
```rust
#[test]
fn test_savepoint_rollback() -> Result<()> {
    let connection = Connection::open_memory(Some("test"));

    connection.with_savepoint("test_save", || {
        // Operations that might fail
        connection.exec("INSERT ...")?()?;
        Ok(())
    })?;

    // Verify state after savepoint
    Ok(())
}
```

**GPUI Testing:**
- Use `TestAppContext` for GPUI tests
- Prefer GPUI executor timers: `cx.background_executor().timer(duration).await`
- Avoid `smol::Timer::after(...)` in GPUI tests

## Existing Test Files

**sqlez crate:**
- `/Users/fk/Devlopment/uavred/crates/sqlez/src/savepoint.rs` - Savepoint rollback tests
- `/Users/fk/Devlopment/uavred/crates/sqlez/src/migrations.rs` - Migration system tests
- `/Users/fk/Devlopment/uavred/crates/sqlez/src/thread_safe_connection.rs` - Connection tests
- `/Users/fk/Devlopment/uavred/crates/sqlez/src/connection.rs` - Core connection tests

**db crate:**
- `/Users/fk/Devlopment/uavred/crates/db/src/db.rs` - Database initialization tests
- `/Users/fk/Devlopment/uavred/crates/db/src/kvp.rs` - Key-value store tests

**assets_ui crate:**
- `/Users/fk/Devlopment/uavred/crates/assets_ui/src/config/zone_config.rs` - Zone config tests

## Test Gaps

**Untested areas identified:**
- UI components (GPUI panels) - no UI tests detected
- Agent system (`crates/agent`) - no tests detected
- Scanner modules (`crates/scanner`) - no tests detected
- Core business logic (`crates/core`) - no tests detected
- Dashboard UI - no tests detected
- Vulns UI - no tests detected

**Recommendation:**
- Add unit tests for agent scheduler and executor
- Add tests for scanner modules
- Consider GPUI test harness for UI components
- Add integration tests for database operations

---

*Testing analysis: 2026-01-29*
