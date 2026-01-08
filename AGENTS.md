# AI Commit - Agent Guidelines

## Build and Development Commands

### Building
```bash
cargo build                 # Debug build
cargo build --release       # Optimized release build
```

### Testing
```bash
cargo test                  # Run all tests
cargo test -- test_name     # Run a specific test
cargo test --lib            # Run library tests only
cargo test -- --nocapture   # Show print output during tests
```

### Code Quality
```bash
cargo fmt                   # Format code according to Rust style
cargo fmt -- --check        # Check formatting without modifying
cargo clippy                # Run linter for code quality
cargo clippy -- -D warnings # Treat warnings as errors
```

### Development
```bash
cargo run                   # Run the application in debug mode
cargo run -- --help         # Run with command-line arguments
```

## Code Style Guidelines

### Naming Conventions
- **Functions and variables**: `snake_case`
- **Types and structs**: `PascalCase`
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Modules**: `snake_case` (directory names)
- **File names**: `snake_case.rs`

### Imports
- Group imports at the top of the file
- Standard library imports first: `use std::{fs, io::Write, process::Command};`
- External crate imports second: `use clap::Parser;`
- Internal module imports third: `use crate::cli::{Cli, ConfigCmd};`
- Use braces for multiple imports from same crate: `use std::{fs, path::PathBuf};`

### Structs and Enums
- Use `#[derive(Debug, Default)]` on structs that benefit from it
- Use `#[derive(Serialize, Deserialize, Debug)]` for data structures
- Keep struct fields public if accessed by other modules
- Use `Option<String>` for optional configuration values

### Async/Await
- Use `#[tokio::main]` for async main functions
- Mark async functions with `async fn`
- Use `.await` at the end of method chains
- Use `Result<T, String>` for async error returns

### Error Handling
- Use `Result<T, String>` for error returns
- Use `.expect()` only when failure should be impossible
- Use `.map_err()` to convert errors: `.map_err(|e| format!("Failed to send request: {}", e))`
- Use early returns for error checks: match/unwrap or if let patterns
- Print errors with `eprintln!()`

### Formatting
- Use block comments for major sections:
  ```rust
  // ===================================================================
  // Command-line Interface
  // ===================================================================
  ```
- Line length: Keep reasonable (not too long, but no strict limit)
- Use `cargo fmt` for automatic formatting
- Space after commas, no space before
- Use `format!()` for string interpolation

### Clap CLI
- Use `#[derive(Parser)]` for CLI structs
- Use `#[clap(short, long)]` for flags with both short and long forms
- Use `#[clap(help = "...")]` for descriptions
- Use `#[clap(subcommand)]` for subcommands
- Mark internal flags with `#[clap(hide = true)]`

### Serde
- Use `#[derive(Serialize, Deserialize)]` for serializable types
- Use `#[serde(rename_all = "...")]` if field names need mapping
- Use `toml::to_string_pretty()` for TOML serialization

### Configuration
- Store config in `~/.config/ai-commit/config.toml`
- Use `directories::ProjectDirs::from("com", "github", "ai-commit")`
- Hide sensitive values (API keys) when displaying config

### Git Operations
- Use `std::process::Command` for external git commands
- Check `output.stdout` for command results
- Use `String::from_utf8_lossy()` for byte-to-string conversion

### HTTP Requests (reqwest)
- Create client with `Client::new()`
- Chain methods: `.post(url).bearer_auth(key).json(&request).send().await`
- Check `res.status()` before parsing body
- Use `.text().await` to get response body as string
- Use `serde_json::from_str::<T>()` for JSON parsing

### Pattern Matching
- Use `match` for enum variants
- Use `if let` for optional value extraction
- Use `unwrap_or_else()` with closure for default values
- Use `.or()` for Option chaining

### Constants
- Default API URL: `https://api.openai.com/v1/chat/completions`
- Default model: `gpt-3.5-turbo`
- Default language: `en`

### File I/O
- Use `fs::read_to_string()` for reading files
- Use `fs::write()` for writing files
- Use `tempfile::Builder::new()` for temporary files
- Clean up temporary files after use

## Project Structure
- `src/main.rs`: Entry point, parses CLI and dispatches commands
- `src/cli.rs`: CLI argument definitions (Clap)
- `src/config.rs`: Configuration loading and saving
- `src/ai_commit.rs`: AI API integration and commit message generation
- `src/lib.rs`: Library exports and core logic

## Dependencies
- `tokio`: Async runtime (full features)
- `reqwest`: HTTP client (json feature)
- `clap`: CLI parsing (cargo and derive features)
- `serde`: Serialization (derive feature)
- `serde_json`: JSON handling
- `toml`: TOML config parsing
- `directories`: Cross-platform config directory
- `tempfile`: Temporary file handling

## Before Making Changes
1. Run `cargo fmt` to format code
2. Run `cargo clippy` to check for issues
3. Run `cargo test` to ensure tests pass
4. Build with `cargo build --release` to verify compilation
