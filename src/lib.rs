mod wit;

use wasi_nn::{self, GraphExecutionContext};

const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const fn version() -> &'static str {
    VERSION
}

pub fn inference() -> String {
    "something".to_string()
}
