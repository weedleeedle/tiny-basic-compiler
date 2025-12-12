//! Lexer module that parses newlines.

use crate::lexer::{LexerModule, LexerModuleResult, LexerModuleSuccessResult};
use crate::lang::Token;

pub struct NewlineLexerModule();

impl LexerModule for NewlineLexerModule
{
    type Language = Token;

    fn parse_stream<'a>(&mut self, stream: &'a str) -> LexerModuleResult<'a, Self::Language>
    {
        if stream.starts_with('\n')
        {
            return LexerModuleResult::TokenSuccess(
                LexerModuleSuccessResult
                {
                    remainder: &stream[1..],
                    token: Token::NewLine,
                }
            );
        }
        else
        {
            return LexerModuleResult::TokenIgnored
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_newline_lexer_module()
    {
        let mut lexer_module = NewlineLexerModule();
        let input_stream = "\nInput";
        let token = lexer_module.parse_stream(input_stream);
        assert!(token.is_success());
        let token = token.unwrap();
        assert_eq!(token.token, Token::NewLine);
        assert_eq!(token.remainder, "Input");
    }

    #[test]
    fn test_newline_lexer_module_ignores_non_newline_char()
    {
        let mut lexer_module = NewlineLexerModule();
        let input_stream = "Hi :)";
        let token = lexer_module.parse_stream(input_stream);
        assert!(token.is_ignored());
    }
}

