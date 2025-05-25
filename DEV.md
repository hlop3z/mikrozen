# Cargo Commands Cheatsheet

## Development

```bash
# Build the project
cargo build --target wasm32-wasip1 --release # Build WASM

# Run tests
cargo test --lib                    # Run all tests
cargo test --lib --features decimal # Run tests with decimal feature

# Check and fix
cargo check                   # Check for compile errors
cargo fmt                     # Format code
cargo clippy                  # Run linter
```

## Dependencies

```bash
# Update dependencies
cargo update                 # Update all dependencies
cargo update -p serde        # Update specific dependency

# Show dependency tree
cargo tree                    # Show all dependencies
cargo tree --features decimal # Show dependencies with decimal feature
```

## Publishing

```bash
# Before publishing
cargo login                   # Login to crates.io
cargo package                 # Create package without publishing
cargo publish --dry-run      # Test publishing process

# Publish
cargo publish                 # Publish to crates.io
```

## Documentation

```bash
# Generate and view docs
cargo doc                     # Generate documentation
cargo doc --open             # Generate and open in browser
cargo doc --no-deps          # Generate docs for your crate only
```
