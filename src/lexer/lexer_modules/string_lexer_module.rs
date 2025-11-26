//! Lexer module for parsing strings

use crate::lexer::{LexerModule, LexerModuleResult, LexerModuleSuccessResult};

pub struct StringLexerModule();

impl LexerModule for StringLexerModule
{
    fn parse_stream<'a>(&mut self, stream: &'a str) -> LexerModuleResult<'a> {
        // We only handle this token if it starts with a "
        if !stream.starts_with("\"")
        {
            return LexerModuleResult::TokenIgnored;
        }

        // If it *does*, we parse everything up to the next " character.
        // This will return us everything inside the string, followed by an empty string for the 
        let end_quote_pos = &stream[1..].find('"');
        if end_quote_pos.is_none()
        {
            return LexerModuleResult::TokenFailed(anyhow::anyhow!("Expected closing \" character!"));
        }

        let end_quote_pos = end_quote_pos.unwrap() + 1;
        // We expect at LEAST one element. Even if it's the empty string.
        // Idk what we should do if we don't get that. Just fail?
        let string_contents = &stream[1..end_quote_pos];

        LexerModuleResult::TokenSuccess(
            LexerModuleSuccessResult
            {
                // We have to add one to get rid of the end quote. Idk why, I'm dumb.
                remainder: &stream[end_quote_pos+1..],
                token: crate::lexer::Token::String(string_contents.to_owned()),
            })
    }
}

#[cfg(test)]
mod tests
{
    use crate::lexer::Token;

    use super::*;
    #[test]
    fn test_parse_string_works()
    {
        let mut lexer_module = StringLexerModule();
        let input_stream = "\"This is a string\"";
        let token = lexer_module.parse_stream(&input_stream);
        assert!(token.is_success());
        let token = token.unwrap();
        assert_eq!(token.token, Token::String(String::from("This is a string")));
        assert_eq!(token.remainder, "");
    }

    #[test]
    fn test_parse_string_with_remainder()
    {
        let mut lexer_module = StringLexerModule();
        let input_stream = "\"This is a string\" followed by a non-string";
        let token = lexer_module.parse_stream(&input_stream);
        assert!(token.is_success());
        let token = token.unwrap();
        assert_eq!(token.token, Token::String(String::from("This is a string")));
        assert_eq!(token.remainder, " followed by a non-string");
    }

    #[test]
    fn test_parse_invalid_string()
    {
        let mut lexer_module = StringLexerModule();
        let input_stream = "\"This is a badly formatted string";
        let token = lexer_module.parse_stream(&input_stream);
        assert!(token.is_failure());
    }

    #[test]
    fn test_parse_not_string()
    {
        let mut lexer_module = StringLexerModule();
        let input_stream = "This is not a string";
        let token = lexer_module.parse_stream(&input_stream);
        assert!(token.is_ignored());
    }
}
