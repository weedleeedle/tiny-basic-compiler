use crate::lang::token::Keyword;
use crate::lang::token::Token;
use crate::lexer::LexerModuleSuccessResult;

use std::str::FromStr;
use crate::lexer::LexerModuleResult;
use crate::lexer::LexerModule;

pub struct KeywordLexerModule();

impl LexerModule for KeywordLexerModule
{
    type Language = Token;

    fn parse_stream<'a>(&mut self, stream: &'a str) -> LexerModuleResult<'a, Self::Language>
    {
        let token = get_first_word(stream); 
        if token.is_none()
        {
            return LexerModuleResult::TokenIgnored;
        }
        let token = token.unwrap();
        let remainder = &stream[stream.find(token).unwrap()+token.len()..];

        let keyword: Result<Keyword, ()> = Keyword::from_str(&token);
        if keyword.is_err()
        {
            return LexerModuleResult::TokenIgnored;
        }
        let keyword = keyword.unwrap();
        LexerModuleResult::TokenSuccess(LexerModuleSuccessResult
        {
            remainder,
            token: Token::Keyword(keyword)
        })
    }
}

/// Gets the first word (up to the first unicode whitespace).
/// Returns [None] if the string is empty or all whitespace.
/// Returns [Some] containing the first word otherwise.
fn get_first_word(string: &str) -> Option<&str>
{
    string.split_whitespace().next()
}

#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    fn test_get_first_word_returns_first_word()
    {
        let s = "This is a string";
        let result = get_first_word(s);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "This");
    }

    #[test]
    fn test_get_first_word_returns_none_empty_string()
    {
        let s = "";
        assert!(get_first_word(s).is_none());
    }

    #[test]
    fn test_get_first_word_returns_none_all_whitespace()
    {
        let s = "          ";
        assert!(get_first_word(s).is_none());
    }

    #[test]
    fn test_get_first_word_returns_first_word_before_newline()
    {
        let s = "Hello\nWorld";
        let result = get_first_word(s);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "Hello");
    }

    #[test]
    fn test_valid_keyword_lexer_module()
    {
        let s = String::from("print \"Hello World!\"");
        let mut lexer_module = KeywordLexerModule();
        let result = lexer_module.parse_stream(&s);
        assert!(result.is_success());
        assert_eq!(result.unwrap().token, Token::Keyword(Keyword::Print))
    }

    #[test]
    fn test_invalid_keyword_lexer_module()
    {
        let s = String::from("This is not a keyword");
        let mut lexer_module = KeywordLexerModule();
        let result = lexer_module.parse_stream(&s);
        assert!(result.is_ignored());
    }

    #[test]
    fn test_empty_string_keyword_lexer_module()
    {
        let s = String::new();
        let mut lexer_module = KeywordLexerModule();
        let result = lexer_module.parse_stream(&s);
        assert!(result.is_ignored());
    }

    #[test]
    fn test_valid_keyword_lexer_module_gets_correct_token()
    {
        let keywords = [
            Keyword::Print,
            Keyword::If,
            Keyword::Then,
            Keyword::Goto,
            Keyword::Input,
            Keyword::Let,
            Keyword::GoSub,
            Keyword::Return,
            Keyword::Clear,
            Keyword::List,
            Keyword::Run,
            Keyword::End
        ];

        let s = String::from("print if then goto input let gosub return clear list run end");
        let mut remainder: &str = &s;
        let mut lexer_module = KeywordLexerModule();
        for keyword in keywords
        {
            let result = lexer_module.parse_stream(remainder);
            let result = result.unwrap();
            match result.token
            {
                // We should only match keywords here, nothing else.
                // The keyword we get should match the current keyword we're testing against.
                Token::Keyword(kw) => assert_eq!(kw, keyword),
                _ => assert!(false),
            }
            // We have to give the module a little help to trim out the remainder. In the main
            // lexer we'll have a module dedicated to removing whitespace, or just have the lexer
            // do it itself.
            remainder = result.remainder.trim_start();
        }

        assert!(remainder.is_empty());
    }

    #[test]
    fn test_valid_keyword_with_newline_separates_correctly()
    {
        let s = "CLEAR\n";
        let mut lexer_module = KeywordLexerModule();
        let result = lexer_module.parse_stream(&s);
        assert!(result.is_success());
        let result = result.unwrap();
        assert_eq!(result.remainder, "\n");
    }


    #[test]
    fn test_valid_keyword_with_preceding_space()
    {
        let s = " CLEAR";
        let mut lexer_module = KeywordLexerModule();
        let result = lexer_module.parse_stream(&s);
        assert!(result.is_success());
        let result = result.unwrap();
        assert_eq!(result.remainder, "");
        assert_eq!(result.token, Token::Keyword(Keyword::Clear));
    }

}
