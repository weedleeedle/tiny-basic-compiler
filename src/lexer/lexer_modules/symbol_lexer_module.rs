//! This module parses symbols

use crate::lexer::{LexerModule, LexerModuleResult, LexerModuleSuccessResult, Token, token::Symbol};

pub struct SymbolLexerModule();

impl LexerModule for SymbolLexerModule
{
    fn parse_stream<'a>(&mut self, stream: &'a str) -> crate::lexer::LexerModuleResult<'a> {

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
    use crate::lexer::lexer::LexerBuilder;

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
        let lexer = LexerBuilder::new()
                        .add_module(Box::new(lexer_module))
                        .build(&input_symbols);

        for (token, expected_token) in lexer.into_iter().zip(expected_token.into_iter())
        {
            assert_eq!(token.unwrap(), expected_token);
        }
    }
}

