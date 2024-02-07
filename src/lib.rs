mod wit;

const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const fn version() -> &'static str {
    VERSION
}
