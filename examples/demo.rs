#![cfg_attr(target_arch = "wasm32", no_std)]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::ToString;
use mikrozen::prelude::*;
use serde_json::Value;

fn hello(args: Input) -> Output {
    let name = args.get_str("name");

    #[cfg(feature = "decimal")]
    {
        let price = args.get_decimal("price");
        return response! {
            "message" => format!("Hello, {}", name),
            "success" => true,
            "price" => price,
        };
    }

    #[cfg(not(feature = "decimal"))]
    {
        return response! {
            "message" => format!("Hello, {}", name),
            "success" => true,
        };
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

    #[cfg(feature = "decimal")]
    map.insert(
        "price".to_string(),
        Value::Number(serde_json::Number::from_f64(100.0).unwrap()),
    );

    let input = RouterInput::new(map);
    let out = Router::dispatch("hello", input);
    println!("{:?}", out);
}
