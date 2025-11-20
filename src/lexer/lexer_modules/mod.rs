//! Common lexer modules.
mod keyword_lexer_module;
mod variable_lexer_module;
mod number_lexer_module;
mod symbol_lexer_module;

pub use keyword_lexer_module::KeywordLexerModule;

/// Gets the first word (up to the first unicode whitespace).
/// Returns [None] if the string is empty or all whitespace.
/// Returns [Some] containing the first word otherwise.
fn get_first_word(string: &str) -> Option<&str>
{
    string.split_whitespace().next()
}
