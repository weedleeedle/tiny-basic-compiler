//! The variable lexer module parses variables, which are a single letter A-Z.

use crate::lexer::{LexerModuleResult, Token, Variable};

use super::super::LexerModule;

pub struct VariableLexerModule();

impl LexerModule for VariableLexerModule
{
    fn parse_stream<'a>(&mut self, stream: &'a str) -> Option<crate::lexer::LexerModuleResult<'a>> {
        let first_char = stream.bytes().next()?;
        let variable: Variable = first_char.try_into().ok()?;
        Some(LexerModuleResult
        {
            remainder: &stream[1..],
            token: Token::Variable(variable)
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
            assert!(result.is_some());
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
            assert!(result.is_some());
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
        assert!(result.is_none());
    }
}
