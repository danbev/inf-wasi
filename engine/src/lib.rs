#![allow(dead_code, unused_variables)]
mod engine;

const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const fn version() -> &'static str {
    VERSION
}
