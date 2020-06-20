#![feature(box_patterns)]
extern crate logos;

#[macro_use]
mod macros;
pub mod codegen;
pub mod config;
pub mod errors;
pub mod frontend;

use codegen::emit_lua;
pub use config::PPGAConfig;
use errors::ErrCtx;
use frontend::{lexer, Parser};

pub fn ppga_to_lua<'a>(source: &'a str, config: PPGAConfig) -> Result<String, ErrCtx<'a>> {
    Parser::with_config(config, lexer(source))
        .parse()
        .map(|ast| emit_lua(&ast))
}
