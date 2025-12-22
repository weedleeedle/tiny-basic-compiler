//! This module parses symbols

use crate::lang::Token;
use crate::lang::token::Symbol;
use crate::lexer::{LexerModule, LexerModuleResult, LexerModuleSuccessResult};

pub struct SymbolLexerModule();

impl LexerModule for SymbolLexerModule
{
    type Language = Token;

    fn parse_stream<'a>(&mut self, stream: &'a str) -> LexerModuleResult<'a, Self::Language>
    {
        let first_char = stream.bytes().next();
        if first_char.is_none()
        {
            return LexerModuleResult::TokenIgnored;
        }
        let first_char = first_char.unwrap();
        let symbol: Result<Symbol, _> = first_char.try_into();
        if symbol.is_err()
        {
            return LexerModuleResult::TokenIgnored;
        }
        let symbol = symbol.unwrap();
        LexerModuleResult::TokenSuccess(LexerModuleSuccessResult
        {
            remainder: &stream[1..],
            token: Token::Symbol(symbol),
        })
    }
}

#[cfg(test)]
mod tests
{
    use crate::lang::token::Symbol;
    use crate::lexer::LexerBuilder;

    use super::*;

    #[test]
    fn test_symbol_list() 
    {
        let input_symbols = "<>=+-*/,";
        let expected_token = vec![
            Token::Symbol(Symbol::LessThanSign),
            Token::Symbol(Symbol::GreaterThanSign),
            Token::Symbol(Symbol::EqualsSign),
            Token::Symbol(Symbol::Plus),
            Token::Symbol(Symbol::Minus),
            Token::Symbol(Symbol::Times),
            Token::Symbol(Symbol::Divide),
            Token::Symbol(Symbol::Comma),
        ];

        let lexer_module = SymbolLexerModule();
        let mut lexer = LexerBuilder::new()
                        .add_module(Box::new(lexer_module))
                        .build();

        for (token, expected_token) in lexer.parse_stream(input_symbols).zip(expected_token.into_iter())
        {
            assert_eq!(token.unwrap(), expected_token);
        }
    }
}

