//! This module handles lexing an input file and returning a token stream
//! to be parsed further by the rest of the application.
//! [LexerModule]s return [LexerModuleResult]s. A [lexer::Lexer]
//! is built out of one or more of these [LexerModule]s. 
//!
//! To get started, construct a [lexer::LexerBuider], which is used to create a [lexer::Lexer]

mod lexer;

pub use lexer::*;

/// Information contained when a token is successfully parsed out of an input stream.
///
/// L is the token type that [LexerModule]s should return.
#[derive(Debug)]
pub struct LexerModuleSuccessResult<'a, L>
{
    /// The remainder of the input stream, with the consumed token's input character(s) subtracted
    /// from the slice.
    pub remainder: &'a str,
    /// The token we produced.
    pub token: L
}

/// Type returned by a [LexerModule].
///
/// L is the type that the [LexerModule]s should return.
///
/// Handles the three possible cases:
/// One: The [LexerModule] parses a token out of the start of the input stream successfully.
/// Two: The character pattern at the start of the input stream is not a token we recognize and
/// handle. We return this to allow the [lexer::Lexer] to continue down the chain of [LexerModule]s
/// and find a different module to parse the stream.
/// Three: The character pattern at the start of the input stream *is* a pattern we are supposed to
/// parse, but is somehow invalid. This happens, notably, if a string doesn't end in a double
/// quotation mark as expected, but anytime a module encounters an input string in an invalid
/// format, it should return the [TokenFailed] variant.
#[derive(Debug)]
pub enum LexerModuleResult<'a, L>
{
    /// The input prefix was parsed successfully.
    TokenSuccess(LexerModuleSuccessResult<'a, L>),
    /// The input prefix was not recognized.
    TokenIgnored,
    /// The input prefix was recognized, but failed to follow an expected pattern.
    TokenFailed(anyhow::Error)
}

impl<L> LexerModuleResult<'_, L>
{
    pub fn is_success(&self) -> bool
    {
        match self
        {
            Self::TokenSuccess(_) => true,
            _ => false,
        }
    }

    pub fn is_ignored(&self) -> bool
    {
        match self
        {
            Self::TokenIgnored => true,
            _ => false,
        }
    }

    pub fn is_failure(&self) -> bool
    {
        match self
        {
            Self::TokenFailed(_) => true,
            _ => false,
        }
    }

}

impl<'a, L> LexerModuleResult<'a, L>
{
    pub fn unwrap(self) -> LexerModuleSuccessResult<'a, L>
    {
        match self
        {
            Self::TokenSuccess(result) => result,
            _ => panic!("Expected LexerModuleResult to be TokenSuccess!")
        }
    }

    pub fn unwrap_err(self) -> anyhow::Error
    {
        match self
        {
            Self::TokenFailed(err) => err,
            _ => panic!("Expected LexerModuleResult to be TokenFailed")
        }
    }
}


pub trait LexerModule
{
    type Language;
    fn parse_stream<'a>(&mut self, stream: &'a str) -> LexerModuleResult<'a, Self::Language>;
}


