//! This module defines the tokenized version of the language

use std::str::FromStr;

use derive_more::Into;
use thiserror::Error;

/// A token of some kind
#[derive(Debug, PartialEq, Eq)]
pub enum Token
{
    Keyword(Keyword),
    Variable(Variable),
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
#[derive(Debug, PartialEq, Eq, Into, Copy, Clone)]
pub struct Variable(u8);

#[derive(Debug, Error)]
pub enum VariableFromU8Error
{
    #[error("Variable character out of range, must be an ASCII character between A and Z, upper case or lowercase.")]
    CharacterOutOfRange,
}

impl TryFrom<u8> for Variable
{
    type Error = VariableFromU8Error;

    /// Attempts to convert a u8 into a [Variable].
    ///
    /// A u8 can only be converted into a [Variable] if it represents an ASCII character between
    /// 'A' and 'Z' (inclusive) or 'a' and 'z' (inclusive). Otherwise the conversion failes and a
    /// [VariableFromU8Error] is returned.
    ///
    /// # Arguments
    ///
    /// * `value` - A u8 representing a single ASCII character or byte. Must be a character
    /// between 'A'-'Z' or 'a'-'z'
    ///
    /// # Examples
    ///
    /// ```
    /// # use tiny_basic_compiler::lexer::Variable;
    /// # use tiny_basic_compiler::lexer::VariableFromU8Error;
    /// let variable: Result<Variable, VariableFromU8Error> = b'A'.try_into();
    /// assert!(variable.is_ok());
    /// let variable = variable.unwrap();
    /// let variable_u8: u8 = variable.into();
    /// assert_eq!(variable_u8, 0);
    /// let variable: Result<Variable, VariableFromU8Error> = 0.try_into();
    /// assert!(variable.is_err());
    /// ```
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value
        {
            x @ b'A'..=b'Z' => Ok(Self(x - b'A')),
            x @ b'a'..=b'z' => Ok(Self(x - b'a')),
            _ => Err(VariableFromU8Error::CharacterOutOfRange),
        }
    }
}

/// All of the accepted symbols by the language?
/// We don't want to interpret here, just parse.
#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, Error)]
pub enum SymbolFromStrError
{
    #[error("The provided symbol was not recognized as a valid symbol of the language")]
    UnrecognizedSymbol,
}

impl TryFrom<u8> for Symbol
{
    type Error = SymbolFromStrError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value
        {
            b'<' => Ok(Self::LessThanSign),
            b'>' => Ok(Self::GreaterThanSign),
            b'=' => Ok(Self::EqualsSign),
            b'+' => Ok(Self::Plus),
            b'-' => Ok(Self::Minus),
            b'*' => Ok(Self::Times),
            b'/' => Ok(Self::Divide),
            b',' => Ok(Self::Comma),
            _ => Err(Self::Error::UnrecognizedSymbol)
        }
    }
}

