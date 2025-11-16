use super::get_first_word;
use std::str::FromStr;
use super::super::{Keyword, LexerModuleResult, Token};
use super::super::LexerModule;

pub struct KeywordLexerModule();

impl LexerModule for KeywordLexerModule
{
    fn parse_stream<'a>(&mut self, stream: &'a str) -> Option<LexerModuleResult<'a>> {
        let token = get_first_word(stream); 
        if token.is_none()
        {
            return None;
        }
        let token = token.unwrap();
        let remainder = &stream[token.len()..];

        let keyword: Result<Keyword, ()> = Keyword::from_str(&token);
        if keyword.is_err()
        {
            return None;
        }
        let keyword = keyword.unwrap();
        Some(LexerModuleResult
        {
            remainder,
            token: Token::Keyword(keyword)
        })
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_valid_keyword_lexer_module()
    {
        let s = String::from("print \"Hello World!\"");
        let mut lexer_module = KeywordLexerModule();
        let result = lexer_module.parse_stream(&s);
        assert!(result.is_some());
    }

    #[test]
    fn test_invalid_keyword_lexer_module()
    {
        let s = String::from("This is not a keyword");
        let mut lexer_module = KeywordLexerModule();
        let result = lexer_module.parse_stream(&s);
        assert!(result.is_none());
    }

    #[test]
    fn test_empty_string_keyword_lexer_module()
    {
        let s = String::new();
        let mut lexer_module = KeywordLexerModule();
        let result = lexer_module.parse_stream(&s);
        assert!(result.is_none());
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
            println!("Remainder: {}", remainder);
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
}
