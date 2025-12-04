//! Represents the [abstract syntax tree](https://en.wikipedia.org/wiki/Abstract_syntax_tree) of
//! Tiny BASIC.

use derive_more::Into;
use thiserror::Error;

use crate::lexer::Symbol;

/// This node represents a line in BASIC.
pub struct Line
{
    line_number: Option<usize>,
    statement: Statement,
}

/// A single statement, which is one of the language's keywords plus any arguments it takes.
pub enum Statement
{
    Print(ExprList),
    If(IfData),
    Goto(Expression),
    Input(VariableList),
    Let(LetData),
    GoSub(Expression),
    Return,
    Clear,
    List,
    Run,
    End
}

pub struct VariableList
{
    variable: Variable,
    cons: Vec<Variable>
}

pub struct ExprList 
{
    expression: ExprListItem,
    cons: Vec<ExprListItem>,
}


pub enum ExprListItem
{
    String(String),
    Expression(Expression),
}

pub struct IfData
{
    l_expression: Expression,
    relop: RelOpSymbol,
    r_expression: Expression
}

pub struct LetData
{
    variable: Variable,
    expression: Expression
}

/// Represents an expression.
pub struct Expression 
{
    /// An expression can start with a + or -
    operator_prefix: Option<ExpressionPrefix>,
    term: Term,
    cons: Vec<ExpressionElement>
}

pub struct ExpressionElement
{
    /// Elements with multiple terms must be combined with + or -
    operator_prefix: ExpressionPrefix,
    term: Term,
}

pub struct Term
{
    factor: Factor,
    cons: Vec<TermElement>
}

pub struct TermElement
{
    prefix: TermPrefix,
    factor: Factor,
}

pub enum Factor
{
    Variable(Variable),
    Number(usize),
    Expression(Box<Expression>),
}

/// A + or - used to connect expression terms.
pub enum ExpressionPrefix
{
    Positive,
    Negative,
}

pub enum TermPrefix
{
    Multiply,
    Divide
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
    /// # use tiny_basic_compiler::parser::ast::Variable;
    /// # use tiny_basic_compiler::parser::ast::VariableFromU8Error;
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

pub enum RelOpSymbol
{
    LessThan,
    LessThanOrEqual,
    Equal,
    GreaterThan,
    GreaterThanOrEqual,
}

impl TryFrom<&[Symbol]> for RelOpSymbol
{
    type Error = ();

    /// We attempt to create a [RelOpSymbol] from a list of [Symbol]s.
    /// This only works if the [Symbol]s are of the expected types, obviously. Otherwise it just
    /// fails.
    fn try_from(value: &[Symbol]) -> Result<Self, Self::Error> {
        match value
        {
            [Symbol::LessThanSign] => Ok(Self::LessThan),
            [Symbol::LessThanSign, Symbol::EqualsSign] => Ok(Self::LessThanOrEqual),
            [Symbol::EqualsSign] => Ok(Self::Equal),
            [Symbol::GreaterThanSign] => Ok(Self::GreaterThan),
            [Symbol::GreaterThanSign, Symbol::EqualsSign] => Ok(Self::GreaterThanOrEqual),
            _ => Err(())
        }
    }
}
