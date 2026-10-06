pub mod cli;
pub mod model;
pub mod parser;
pub mod stream;

pub use model::NormalizedLog;
pub use parser::{parse_log_line, ParserOptions};
pub use stream::process_stream;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
