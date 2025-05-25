//! # mikrozen
//!
//! A minimal, extensible, and WASM-first micro-framework for routing and response handling.
//!
//! ## Features
//! - Simple router and response macros
//! - WASM and no_std compatible
//! - Uses [mimalloc](https://github.com/microsoft/mimalloc) as the global allocator for performance and reliability
//!
//! ## Example
//!
//! ```rust
//! #![no_std]
//! extern crate alloc;
//!
//! use mikrozen::prelude::*;
//!
//! fn hello(args: Input) -> Output {
//!     let name = args.get_str("name");
//!     response! {
//!         "message" => format!("Hello, {}", name),
//!         "success" => true,
//!     }
//! }
//!
//! router! {
//!     "hello" => hello,
//! }
//!
//! # fn main() {}
//! ```
//!
//! ## Global Allocator
//!
//! This crate uses [mimalloc](https://github.com/microsoft/mimalloc) as the global allocator for improved performance and reliability in production environments.
#![no_std]

extern crate alloc;
extern crate mimalloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
#[allow(unused_imports)]
use alloc::string::ToString;
use alloc::vec::Vec;
use serde_json::Value;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

pub mod prelude {
    pub use super::{response, router, Input, Output, RouterInput};
    pub use alloc::collections::BTreeMap;
    pub use alloc::format;
    pub use alloc::string::ToString;
    pub use serde_json::Value;
}

pub type Input = RouterInput;
pub type Output = Value;

pub struct RouterInput(pub BTreeMap<String, Value>);

impl RouterInput {
    pub fn new(map: BTreeMap<String, Value>) -> Self {
        Self(map)
    }
    #[cfg(feature = "decimal")]
    pub fn get_decimal(&self, key: &str) -> rust_decimal::Decimal {
        use rust_decimal::prelude::FromStr;
        match self.0.get(key) {
            Some(Value::Number(n)) => rust_decimal::Decimal::from_f64(n.as_f64().unwrap_or(0.0))
                .unwrap_or(rust_decimal::Decimal::ZERO),
            Some(Value::String(s)) => {
                rust_decimal::Decimal::from_str(s).unwrap_or(rust_decimal::Decimal::ZERO)
            }
            _ => rust_decimal::Decimal::ZERO,
        }
    }
    pub fn get_i64(&self, key: &str) -> i64 {
        self.0.get(key).and_then(Value::as_i64).unwrap_or(0)
    }
    pub fn get_f64(&self, key: &str) -> f64 {
        self.0.get(key).and_then(Value::as_f64).unwrap_or(0.0)
    }
    pub fn get_bool(&self, key: &str) -> bool {
        self.0.get(key).and_then(Value::as_bool).unwrap_or(false)
    }
    pub fn get_str(&self, key: &str) -> &str {
        self.0.get(key).and_then(Value::as_str).unwrap_or("")
    }
    pub fn get_array(&self, key: &str) -> Vec<Value> {
        self.0
            .get(key)
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default()
    }
    pub fn get_object(&self, key: &str) -> BTreeMap<String, Value> {
        self.0
            .get(key)
            .and_then(Value::as_object)
            .map(|m| m.clone().into_iter().collect())
            .unwrap_or_default()
    }
    pub fn get_value(&self, key: &str) -> Option<&Value> {
        self.0.get(key)
    }
    pub fn has(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }
    pub fn raw(&self) -> &BTreeMap<String, Value> {
        &self.0
    }
}

#[macro_export]
macro_rules! router {
    ( $( $route:expr => $handler:ident ),* $(,)? ) => {
        pub struct Router;
        impl Router {
            pub fn dispatch(path: &str, input: $crate::Input) -> $crate::Output {
                match path {
                    $( $route => $handler(input), )*
                    _ => $crate::response! {
                        "error" => ::alloc::format!("Route not found: {}", path),
                        "success" => false,
                    },
                }
            }
        }
    };
}

#[macro_export]
macro_rules! response {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {{
        let mut map = ::serde_json::Map::new();
        $( map.insert($key.to_string(), ::serde_json::json!($value)); )*
        ::serde_json::Value::Object(map)
    }};
}

/// # Example
///
/// ```rust
/// use mikrozen::prelude::*;
///
/// fn hello(args: Input) -> Output {
///     let name = args.get_str("name");
///     response! {
///         "message" => alloc::format!("Hello, {}", name),
///         "success" => true,
///     }
/// }
///
/// router! {
///     "hello" => hello,
/// }
///
/// // Usage:
/// // let input = RouterInput::new(BTreeMap::new());
/// // let out = Router::dispatch("hello", input);
/// ```

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::collections::BTreeMap;
    use alloc::format;

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

    #[test]
    fn test_hello_route() {
        let mut map = BTreeMap::new();
        map.insert("name".to_string(), Value::String("World".to_string()));
        let input = RouterInput::new(map);
        let out = Router::dispatch("hello", input);
        assert_eq!(out["message"], "Hello, World");
        assert_eq!(out["success"], true);
    }

    #[test]
    fn test_missing_route() {
        let input = RouterInput::new(BTreeMap::new());
        let out = Router::dispatch("missing", input);
        assert_eq!(out["success"], false);
        assert!(out["error"].as_str().unwrap().contains("Route not found"));
    }
}
