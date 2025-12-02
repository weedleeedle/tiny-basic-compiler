//! The variable lexer module parses variables, which are a single letter A-Z.

use crate::lexer::{LexerModuleResult, Token, Variable, VariableFromU8Error};

use super::super::LexerModule;

pub struct VariableLexerModule();

impl LexerModule for VariableLexerModule
{
    fn parse_stream<'a>(&mut self, stream: &'a str) -> crate::lexer::LexerModuleResult<'a> {
        let first_char = stream.bytes().next();
        if first_char.is_none()
        {
            return LexerModuleResult::TokenIgnored;
        }
        let first_char = first_char.unwrap();
        // If the first character is NOT a-z, we just ignore it.
        // We don't consider it to be a failure.
        let variable: Result<Variable, VariableFromU8Error> = first_char.try_into();
        if variable.is_err()
        {
            return LexerModuleResult::TokenIgnored
        }

        LexerModuleResult::TokenSuccess(
            crate::lexer::LexerModuleSuccessResult
        {
            remainder: &stream[1..],
            token: Token::Variable(variable.unwrap())
        })
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    pub fn test_parse_stream_lowercase_variable()
    {
        // Whitespace doesn't matter since we parse one character at a time.
        // For better or for worse? I don't know.
        // Maybe we want VariableLexerModule to be greedy and fail if a variable name is too long.
        // Idk.
        let s = String::from("abcdefghijklmnopqrstuvwxyz"); 
        let mut remainder: &str = &s;
        let mut lexer_module = VariableLexerModule();
        for i in 0..26
        {
            let result = lexer_module.parse_stream(remainder);
            assert!(result.is_success());
            let result = result.unwrap();
            let token = result.token;
            match token
            {
                Token::Variable(x) => assert_eq!(Into::<u8>::into(x), i),
                _ => panic!("Expected token to be a variable!"),
            }
            remainder = result.remainder;
        }
        assert!(remainder.is_empty());
    }

    #[test]
    pub fn test_parse_stream_uppercase_variable()
    {
        let s = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ"); 
        let mut remainder: &str = &s;
        let mut lexer_module = VariableLexerModule();
        for i in 0..26
        {
            let result = lexer_module.parse_stream(remainder);
            assert!(result.is_success());
            let result = result.unwrap();
            let token = result.token;
            match token
            {
                Token::Variable(x) => assert_eq!(Into::<u8>::into(x), i),
                _ => panic!("Expected token to be a variable!"),
            }
            remainder = result.remainder;
        }
        assert!(remainder.is_empty());
    }

    #[test]
    pub fn test_parse_stream_fails_on_non_alphabetic_character()
    {
        let s = String::from("0");
        let mut lexer_module = VariableLexerModule();
        let result = lexer_module.parse_stream(&s);
        assert!(result.is_ignored());
    }

    pub fn test_parse_stream_fails_on_newline_character()
    {
        let s = String::from("\n");
        let mut lexer_module = VariableLexerModule();
        let result = lexer_module.parse_stream(&s);
        assert!(result.is_ignored());
    }
}
