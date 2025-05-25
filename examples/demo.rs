#![cfg_attr(target_arch = "wasm32", no_std)]
extern crate alloc;

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

#[cfg_attr(target_arch = "wasm32", export_name = "main")]
fn main() {
    let mut map = BTreeMap::new();
    map.insert("name".to_string(), Value::String("World".to_string()));
    let input = RouterInput::new(map);
    let out = Router::dispatch("hello", input);
    println!("{:?}", out);
}
