//! This lexer module converts a sequence of digits into a number.
//!
//! It doesn't take into account sign or commas or anything.

use crate::lexer::{LexerModule, LexerModuleResult, LexerModuleSuccessResult, Token};

pub struct NumberLexerModule();

impl LexerModule for NumberLexerModule
{
    fn parse_stream<'a>(&mut self, stream: &'a str) -> crate::lexer::LexerModuleResult<'a> {
        let number_str = get_all_digits_at_start(stream);
        if number_str.is_empty()
        {
            return LexerModuleResult::TokenIgnored;
        }

        // This might actually be a failure, idk.
        // This shouldn't happen I think???
        let number: Result<usize, _> = number_str.parse();
        if number.is_err()
        {
            return LexerModuleResult::TokenIgnored;
        }

        LexerModuleResult::TokenSuccess(
            LexerModuleSuccessResult
            {
                remainder: &stream[number_str.len()..],
                token: Token::Number(number.unwrap()),
            }
        )
    }
}

fn get_all_digits_at_start(stream: &str) -> &str
{
    // Find the first NON digit character
    let index = stream.find(|c: char| !c.is_ascii_digit());
    return &stream[0..index.unwrap_or(0)];

}

#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    fn test_parse_number_correctly()
    {
        let mut lexer_module = NumberLexerModule();
        let result = lexer_module.parse_stream("1234asdfg");
        assert!(result.is_success());
        let result = result.unwrap();
        assert_eq!(result.token, Token::Number(1234));
        assert_eq!(result.remainder, "asdfg");
    }

    #[test]
    fn test_parse_non_number()
    {
        let mut lexer_module = NumberLexerModule();
        let result = lexer_module.parse_stream("this is not a number");
        assert!(result.is_ignored());
    }
}
