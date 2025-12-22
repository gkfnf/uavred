# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview
VibeKanban is a desktop Kanban board application built with Rust and GPUI (the GUI framework from Zed Industries). The application provides a visual task management interface with columns and draggable cards.

## Development Principles

### Test-Driven Development (TDD)
**This project follows TDD principles. All code changes MUST be accompanied by tests.**

- Write tests BEFORE implementing features when possible
- All new functionality must have corresponding tests
- Code that doesn't pass tests should not be considered complete
- Run `cargo test` frequently during development
- Test coverage is a requirement, not optional

### Testing Strategy
- **Unit tests**: Located in `src/` files using `#[cfg(test)]` modules
- **Integration tests**: Located in `tests/` directory
- **Current limitation**: GPUI components require window context, making some UI tests challenging
- **Focus area**: Test business logic (data models, state management) separate from rendering

## Architecture

### Core Components
- **main.rs**: Application entry point that initializes the GPUI app and creates the main window
- **kanban.rs**: Contains the complete Kanban board implementation including:
  - Data models (`Card`, `Column`, `KanbanBoard`)
  - Business logic (adding columns/cards, state management)
  - UI rendering using GPUI's declarative element API

### GPUI Framework
This project uses GPUI, Zed's GUI framework. Key characteristics:
- Declarative UI using `div()` builders with method chaining
- Reactive rendering through `ViewContext`
- Colors defined using `hsla()` and `rgb()` functions
- Styling methods like `.flex()`, `.bg()`, `.rounded_md()`, `.p_3()`, etc.
- The `Render` trait implements the view's rendering logic

### Data Model
- `KanbanBoard` maintains a vector of `Column`s
- Each `Column` contains a vector of `Card`s
- Auto-incrementing IDs for cards and columns
- Drag-and-drop state tracked via `dragging_card` field (currently unused)

## Git Configuration

### Repository
- **Remote**: https://github.com/gkfnf/uavred.git
- **Protocol**: HTTPS (SSH keys not configured)

### Syncing to GitHub
```bash
git add .
git commit -m "message"
git push
```

## Development Commands

### Building
```bash
cargo build
```

### Running
```bash
cargo run
```

### Type Checking
```bash
cargo check
```

### Linting
```bash
cargo clippy
```

### Formatting
```bash
cargo fmt
```

### Testing
```bash
cargo test
```

**Note**: Tests are located in:
- `tests/` directory for integration tests
- `#[cfg(test)]` modules within source files for unit tests

## Dependencies
- **gpui**: GUI framework from Zed (git dependency, main branch)
- **anyhow**: Error handling
- **serde/serde_json**: Serialization support for saving/loading boards (not yet implemented)

## Important Notes

### Build Performance & Caching
**Cargo caches compiled dependencies automatically:**
- **First build**: Downloads and compiles all dependencies (slower, especially for gpui)
- **Subsequent builds**: Only recompiles changed code (much faster)
- **Dependency cache**: `~/.cargo/registry` (downloaded packages) and `target/` (compiled artifacts)
- **Cache invalidation**: Only when `Cargo.toml` dependencies change or `cargo clean` is run
- **Git dependencies**: May rebuild if upstream branch updates

### GPUI Dependency
The project depends on GPUI from the Zed repository's main branch. This means:
- Build times may be longer due to git dependency compilation
- The API may change as Zed evolves
- You need a working internet connection for initial builds
- **macOS requirement**: Full Xcode installation needed (not just Command Line Tools) for Metal shader compilation

### Styling Pattern
GPUI uses a CSS-like API with Rust methods. Common patterns:
- Layout: `.flex()`, `.flex_col()`, `.items_center()`, `.justify_between()`
- Sizing: `.w()`, `.h()`, `.size_full()`, `.px(value)`
- Spacing: `.p_3()` (padding), `.m_2()` (margin), `.mb_1()` (margin-bottom)
- Colors: `.bg()`, `.text_color()`, `.border_color()`
- Effects: `.rounded_md()`, `.shadow_sm()`, `.hover(|style| ...)`

### Current Limitations
- Drag-and-drop functionality is tracked but not implemented
- No persistence (serde is included but save/load not implemented)
- Limited test coverage (basic framework in place)
- Chinese text in initial sample data

## When Making Changes

### Adding New Features
- Follow the existing GPUI element builder pattern
- Keep rendering logic in methods that return `impl IntoElement`
- Use `ViewContext<Self>` for accessing the view context

### State Management
- Mutable state lives in the `KanbanBoard` struct
- UI updates happen through the `Render` trait implementation
- Use `cx.notify()` when state changes require re-rendering (though not shown in current code)

### Styling Consistency
Maintain the existing color scheme:
- Background: `hsla(0.6, 0.1, 0.98, 1.0)`
- Cards: Light purple-ish `hsla(0.55, 0.3, 0.95, 1.0)`
- Each column has its own accent color defined when created
