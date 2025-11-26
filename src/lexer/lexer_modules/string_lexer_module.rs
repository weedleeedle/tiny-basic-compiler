//! Lexer module for parsing strings

use crate::lexer::LexerModule;

pub struct StringLexerModule();

impl LexerModule for StringLexerModule
{
    fn parse_stream<'a>(&mut self, stream: &'a str) -> Option<crate::lexer::LexerModuleResult<'a>> {
        // We only handle this token if it starts with a "
        if !stream.starts_with("\"")
        {
            return None;
        }

        // If it *does*, we parse everything up to the next " character.
        let mut stream_split = &stream[1..].split('"');
        // We expect at LEAST one element.
        // Idk what we should do if we don't get that. Just fail?
        let string = stream_split.next().unwrap()
    }
}
