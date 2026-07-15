pub const RCALC_VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod builtins;
pub mod cli;
pub mod config;
pub mod eval;
pub mod help;
pub mod lexer;
pub mod mcp;
pub mod number;
pub mod parser;
pub mod value;
