//! This module parses symbols

use crate::lexer::{LexerModule, LexerModuleResult, Token, token::Symbol};

pub struct SymbolLexerModule();

impl LexerModule for SymbolLexerModule
{
    fn parse_stream<'a>(&mut self, stream: &'a str) -> Option<crate::lexer::LexerModuleResult<'a>> {

        let first_char = stream.bytes().next()?;
        let symbol: Symbol = first_char.try_into().ok()?;
        Some(LexerModuleResult
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
            assert_eq!(token, expected_token);
        }
    }
}

