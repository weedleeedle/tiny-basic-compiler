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
    /// we consume the self to avoid a duplication.
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
        Self
        {
            lexer_modules: self.lexer_modules,
        }
    }
}

pub struct Lexer<'a>
{
    lexer_modules: Vec<Box<dyn LexerModule>>,
    input_stream: &'a str,
}

impl<'a> IntoIterator for Lexer<'a>
{
    type Item = Token;

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
    fn parse_stream(&mut self) -> Option<Token>
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
    /// Effectively parsing can fail for two reasons.
    /// 1. The stream is empty (halt here, we're done iterating.)
    /// 2. The frontmost symbol was unhandled by any lexer module. (We skip it and move on.)
    ///
    /// Updates our stored position in the [input_stream].
    fn try_parse_first_token(&mut self) -> Option<Token>
    {
        let mut remainder = self.input_stream;
        let token = self.try_each_lexer(remainder);
        if let Some(token) = token.as_ref()
        {
            remainder = token.remainder;
        }
        else
        {
            remainder = &remainder[1..];
        }

        // update input stream to strip the remaining input characters.
        self.input_stream = remainder;
        token.map(|x| x.token)
    }

    fn try_each_lexer(&mut self, stream: &'a str) -> Option<super::LexerModuleResult<'a>>
    {
        for lexer in self.lexer.lexer_modules.iter_mut()
        {
            let result = lexer.as_mut().parse_stream(stream);
            if result.is_some()
            {
                return result;
            }
        }
        return None;
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.parse_stream()
    }
}
