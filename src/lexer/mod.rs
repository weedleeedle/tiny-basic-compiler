use std::str::FromStr;

use derive_more::Into;
use thiserror::Error;

mod lexer;
mod token;
mod lexer_modules;

pub use token::*;

/// Type returned by a [[LexerModule]].
pub struct LexerModuleResult<'a>
{
    pub remainder: &'a str,
    pub token: Token
}

pub trait LexerModule
{
    fn parse_stream<'a>(&mut self, stream: &'a str) -> Option<LexerModuleResult<'a>>;
}


