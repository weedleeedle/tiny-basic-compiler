use std::str::FromStr;

mod lexer;
mod lexer_modules;

/// Type returned by a [[LexerModule]].
pub struct LexerModuleResult<'a>
{
    pub remainder: &'a str,
    pub token: Token
}

pub trait LexerModule
{
    fn parse_stream<'a>(&mut self, stream: &'a str) -> Option<LexerModuleResult<'a>>;
}

pub enum Token
{
    Keyword(Keyword),
    Number(usize),
    String(String),
    Symbol(Symbol),
    NewLine,
}

/// Language keywords, as defined [here](https://en.wikipedia.org/wiki/Tiny_BASIC#Formal_grammar)
#[derive(Debug, PartialEq, Eq)]
pub enum Keyword
{
    Print,
    If,
    Then,
    Goto,
    Input,
    Let,
    GoSub,
    Return,
    Clear,
    List,
    Run,
    End
}

impl FromStr for Keyword
{
    /// Only returns one error: when a string was not one of the expected keywords.
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("print")
        {
            Ok(Self::Print)
        }
        else if s.eq_ignore_ascii_case("if")
        {
            Ok(Self::If)
        }
        else if s.eq_ignore_ascii_case("then")
        {
            Ok(Self::Then)
        }
        else if s.eq_ignore_ascii_case("goto")
        {
            Ok(Self::Goto)
        }
        else if s.eq_ignore_ascii_case("input")
        {
            Ok(Self::Input)
        }
        else if s.eq_ignore_ascii_case("let")
        {
            Ok(Self::Let)
        }
        else if s.eq_ignore_ascii_case("gosub")
        {
            Ok(Self::GoSub)
        }
        else if s.eq_ignore_ascii_case("return")
        {
            Ok(Self::Return)
        }
        else if s.eq_ignore_ascii_case("clear")
        {
            Ok(Self::Clear)
        }
        else if s.eq_ignore_ascii_case("list")
        {
            Ok(Self::List)
        }
        else if s.eq_ignore_ascii_case("run")
        {
            Ok(Self::Run)
        }
        else if s.eq_ignore_ascii_case("end")
        {
            Ok(Self::End)
        }
        else
        {
            Err(())
        }
    }
}

/// A variable is any single letter from A-Z.
/// We'll convert it to 0-25 internally probably?
pub struct Variable(u8);

/// All of the accepted symbols by the language?
/// We don't want to interpret here, just parse.
pub enum Symbol
{
    LessThanSign,
    GreaterThanSign,
    EqualsSign,
    Plus,
    Minus,
    Times,
    Divide,
    Comma,
}

