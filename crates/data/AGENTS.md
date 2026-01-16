# Data Layer

**Purpose**: Central data models, SQLite persistence via sqlez, and repository interfaces.

## OVERVIEW

Defines rich domain models (TaskData, VulnData, AssetNode, etc.), manages SQLite database operations using Zed's `sqlez` crate, and provides repository traits for abstract data access.

## STRUCTURE

```
data/
├── src/
│   ├── models.rs      # Domain models (~988 lines)
│   ├── database.rs   # SQLite connection and queries
│   ├── repository.rs  # Repository trait interfaces
│   └── memory.rs     # In-memory storage
```

## WHERE TO LOOK

| Task | Location |
|------|----------|
| Model definitions | `models.rs` |
| Database schema | `database.rs` (CREATE TABLE statements) |
| CRUD operations | `database.rs` |
| Repository traits | `repository.rs` |
| In-memory stores | `memory.rs` |

## CONVENTIONS

### Model Patterns

All models use serde derive macros for JSON serialization:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskData {
    pub id: usize,
    pub title: String,
    pub status: TaskStatus,
}
```

Enum types implement `Display` for string conversion:
```rust
impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Todo => write!(f, "todo"),
            // ...
        }
    }
}
```

### Database Connection

Uses `sqlez::connection::Connection` with `Arc<Mutex<>>` for thread safety:
```rust
pub struct TasksDatabase {
    connection: Arc<Mutex<Connection>>,
}
```

Database file location: `{dirs::data_dir()}/uavred/tasks.db`

### Repository Pattern

Abstract interfaces for dependency injection and testing:
```rust
pub trait TaskRepository: Send + Sync {
    fn get_tasks(&self, status: TaskStatus) -> Vec<TaskData>;
    fn add_task(&mut self, task: TaskData);
    fn remove_task(&mut self, id: usize);
}
```

### Query Pattern

Lock connection before queries:
```rust
let connection = self.connection.lock().unwrap();
```

## ANTI-PATTERNS

- **Never unwrap Mutex locks in production code** - use `?` with proper error context
- **Don't duplicate model definitions** - use models from `workspace` crate where applicable (e.g., `use workspace::TaskData as WorkspaceTaskData`)
- **Avoid inline SQL strings** - use `indoc::indoc!` for multi-line queries for readability
- **Never use raw SQL string interpolation** - use parameterized queries to prevent SQL injection (sqlez handles this)
- **Don't mix data types** - use consistent types (`usize` for IDs, `String` for text)

## KEY INTEGRATIONS

- **sqlez**: Zed's SQLite wrapper for async-friendly database operations
- **serde**: JSON serialization for API responses and config
- **chrono**: Timestamps (`created_at`, `updated_at`)
- **workspace crate**: Reuse shared types when possible

## NOTES

- Data models are shared across all crates via the `data` crate
- Repository traits enable mock implementations for testing
- Database migrations are currently manual - no automated schema versioning
