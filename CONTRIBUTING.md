# Contributing to Log2JSON

Thank you for contributing!

## Development
```bash
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo run -- --help
```

Please add unit tests under `tests/` for any new log format heuristics or parsing logic.
