//! This module defines the specific shape and grammar of the [Tiny
//! BASIC](https://en.wikipedia.org/wiki/Tiny_BASIC) language.

pub mod ast;
pub mod lexer_modules;
pub mod token;

mod lexer_program_tests;

use crate::{lang::{ast::Program, token::Token}, lexer::{Lexer, LexerBuilder}, parser::{Grammar, ParseEngine}};

use lexer_modules::*;

/// Creates a lexer to parse the tiny basic language.
pub fn create_lexer() -> Lexer<Token>
{
    LexerBuilder::<Token>::new()
        .add_modules(vec![
            Box::new(StringLexerModule()),
            Box::new(KeywordLexerModule()),
            Box::new(NumberLexerModule()),
            Box::new(VariableLexerModule()),
            Box::new(SymbolLexerModule()),
            Box::new(NewlineLexerModule()),
        ])
        .build()
}

