# Agents

## Build & Run
```bash
cargo build --release
./target/release/which <command>...
```

## Project Structure
- Single binary crate: `which` (output binary name)
- Entry point: `src/main.rs`
- Dependency: `which` crate (version 8.0.4)

## Notes
- Uses `which::which()` for exact command lookup (no glob/regex support in current deps)
- Glob pattern support requires enabling `regex` feature on `which` crate and using `which::which_re()`
