//! Defines the core [Lexer] and [LexerBuilder] types.
use crate::lexer::LexerModuleResult;

use super::LexerModule;
use super::Token;

pub struct LexerBuilder
{
    lexer_modules: Vec<Box<dyn LexerModule>>
}

impl LexerBuilder
{
    /// Creates a new [LexerBuilder]
    pub fn new() -> Self
    {
        Self
        {
            lexer_modules: Vec::new()
        }
    }

    /// Builds this [LexerBuilder] into a [Lexer].
    /// Since memory is allocated on the heap for Lexer Modules,
    /// we consume the self to avoid a duplication.le
    ///
    /// We also provide the input stream that we're planning on parsing.
    pub fn build<'a>(self, input_stream: &'a str) -> Lexer<'a>
    {
        Lexer 
        { 
            input_stream,
            lexer_modules: self.lexer_modules,
        }
    }

    /// Adds a [LexerModule] to the Lexer. LexerModules handle the input stream and convert
    /// them to a sequence of tokens.
    pub fn add_module(mut self, module: Box<dyn LexerModule>) -> Self
    {
        self.lexer_modules.push(module);
        self
    }

    /// Adds multiple [LexerModule]s to the Lexer. Doesn't erase existing modules, only appends to
    /// the list of modules.
    pub fn add_modules(mut self, modules: Vec<Box<dyn LexerModule>>) -> Self
    {
        self.lexer_modules.extend(modules);
        self
    }
}

pub struct Lexer<'a>
{
    lexer_modules: Vec<Box<dyn LexerModule>>,
    input_stream: &'a str,
}

impl<'a> IntoIterator for Lexer<'a>
{
    type Item = Result<Token, anyhow::Error>;

    type IntoIter = TokenIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TokenIterator
        {
            input_stream: self.input_stream,
            lexer: self
        }
    }
}

pub struct TokenIterator<'a>
{
    lexer: Lexer<'a>,
    input_stream: &'a str
}

impl<'a> TokenIterator<'a>
{
    /// Produces the first valid token and updates the input stream accordingly.
    fn parse_stream(&mut self) -> Option<Result<Token, anyhow::Error>>
    {
        loop 
        {
            // Handle empty stream and return a none token.
            if self.input_stream.is_empty()
            {
                return None;
            }

            // Otherwise we try and parse the input.
            let token = self.try_parse_first_token();
            // Parse succeeded.
            if token.is_some()
            {
                return token;
            }

            // If the parse failed we loop.
        }
    }

    /// Attempts to extract a token from the start of the string.
    ///
    /// Effectively parsing can fail for three reasons.
    /// 1. The stream is empty (halt here, we're done iterating.)
    /// 2. The frontmost symbol was unhandled by any lexer module. (We skip it and move on.)
    /// 3. A lexer module *attempted* to parse the token but failed.
    ///     This failure means an unrecoverable error, so we want to return the error.
    ///
    /// Updates our stored position in the [input_stream].
    fn try_parse_first_token(&mut self) -> Option<Result<Token, anyhow::Error>>
    {
        let mut remainder = self.input_stream;
        let token = self.try_each_lexer(remainder);
        println!("Token: {:?}", token);
        if token.is_failure()
        {
            // Halt and return the error.
            return Some(Err(token.unwrap_err()));
        }

        if token.is_ignored()
        {
            // If nobody handled this character, silently consume it 
            // and move onto the next character.
            remainder = &remainder[1..];
        }

        if let LexerModuleResult::TokenSuccess(result) = &token
        {
            println!("Remainder: {}", result.remainder);
            remainder = result.remainder;
        }

        // update input stream to strip the remaining input characters.
        self.input_stream = remainder;
        match token
        {
            super::LexerModuleResult::TokenSuccess(success) => Some(Ok(success.token)),
            super::LexerModuleResult::TokenIgnored => None,
            super::LexerModuleResult::TokenFailed(error) => Some(Err(error)),
        }
    }

    /// Returns the result of the 
    fn try_each_lexer(&mut self, stream: &'a str) -> super::LexerModuleResult<'a>
    {
        for lexer in self.lexer.lexer_modules.iter_mut()
        {
            let result = lexer.as_mut().parse_stream(stream);
            // Basically how this works:
            //
            // - If the token was parsed successfully, we return it.
            // - If an error was produced while handling the token, we return the error.
            // - If the token was ignored, we just move onto the next module.
            //
            // Basically as soon as a module tries to parse the token, whether or not it succeeded,
            // we return it.
            if !result.is_ignored()
            {
                return result;
            }
        }
        return super::LexerModuleResult::TokenIgnored;
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    // Parsing the token stream could fail.
    type Item = Result<Token, anyhow::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.parse_stream()
    }
}

#[cfg(test)]
mod tests
{
    use crate::lexer::{LexerModuleResult, LexerModuleSuccessResult};

    use super::*;

    // Dummy lexer module that just returns each token provided to it and consumes one character of
    // an input string at a time.
    pub struct TestLexerModule
    {
        tokens_to_return: std::vec::IntoIter<Token>,
    }

    impl TestLexerModule
    {
        pub fn new(tokens: Vec<Token>) -> Self
        {
            Self
            {
                tokens_to_return: tokens.into_iter()
            }
        }
    }

    impl LexerModule for TestLexerModule
    {
        fn parse_stream<'a>(&mut self, stream: &'a str) -> LexerModuleResult<'a> {
            let token = self.tokens_to_return.next();
            token.map_or(
            LexerModuleResult::TokenIgnored,
            |x| {
                LexerModuleResult::TokenSuccess(
                    LexerModuleSuccessResult {
                        remainder: &stream[1..],
                        token: x 
                })
            })
        }
    }

    #[test]
    fn test_can_build_lexer()
    {
        let lexer = LexerBuilder::new().build("Test");
        assert_eq!(lexer.lexer_modules.len(), 0);
        assert_eq!(lexer.input_stream, "Test");
    }

    #[test]
    fn test_lexer_with_test_lexer()
    {
        let tokens = vec![Token::NewLine];
        let test_lexer_module = TestLexerModule::new(tokens);
        let lexer = LexerBuilder::new()
                    .add_module(Box::new(test_lexer_module))
                    .build("A");

        // Lol I love that we can just turn Vec<Result> into Result<Vec> with .collect().
        // Not sure how I feel about the token stream being an iterator over results but it's the
        // only thing I can think of ig.
        let ret_tokens: Result<Vec<Token>, anyhow::Error> = lexer.into_iter().collect();
        assert!(ret_tokens.is_ok());
        let ret_tokens = ret_tokens.unwrap();
        assert_eq!(ret_tokens.len(), 1);
        assert_eq!(ret_tokens[0], Token::NewLine);
    }
}
