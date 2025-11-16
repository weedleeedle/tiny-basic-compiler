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
    pub fn build(self) -> Lexer
    {
        Lexer 
        { 
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

pub struct Lexer
{
    lexer_modules: Vec<Box<dyn LexerModule>>,
}

impl Lexer
{
    pub fn parse_stream(&mut self, stream: &str) -> Vec<Token>
    {
        let mut tokens: Vec<Token> = Vec::new();
        let mut remainder = stream;
        while !remainder.is_empty()
        {
            let token = self.try_each_lexer(stream);
            if let Some(token) = token
            {
                remainder = token.remainder;
                tokens.push(token.token);
            }
            else
            {
                remainder = &remainder[1..];
            }
        }

        tokens
    }

    fn try_each_lexer<'a>(&mut self, stream: &'a str) -> Option<super::LexerModuleResult<'a>>
    {
        for lexer in self.lexer_modules.iter_mut()
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
