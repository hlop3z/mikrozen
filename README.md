# Mikrozen

A minimal, ergonomic router and JSON response macro for `#![no_std]` WASI Rust plugins.

## Features

- `#![no_std]` + `extern crate alloc`
- Simple string-based routing for WASI plugins
- Ergonomic `response!` macro for JSON output
- Input extraction helpers for common types
- Optional `decimal` feature for `rust_decimal`
- Designed for Go-hosted WASI, not browsers

## Example Usage

```rust
use mikrozen::prelude::*;

fn hello(args: Input) -> Output {
    let name = args.get_str("name");
    response! {
        "message" => format!("Hello, {}", name),
        "success" => true,
    }
}

router! {
    "hello" => hello,
}

// Usage:
// let input = RouterInput::new(BTreeMap::new());
// let out = Router::dispatch("hello", input);
```

## Input Extraction

```rust
let n = args.get_i64("n");
let flag = args.get_bool("flag");
let arr = args.get_array("arr");
let obj = args.get_object("obj");
let val = args.get_value("key");
```

## Decimal Feature

Enable with `--features decimal` to use `get_decimal`:

```rust
let dec = args.get_decimal("amount");
```

## License

BSD 3-Clause
