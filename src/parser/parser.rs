//! This module defines the actual parser implementation that produces an AST from a stream of
//! tokens.

use std::{collections::HashMap, rc::Rc};

use anyhow::Result;

use crate::{lexer::{Keyword, Token}, parser::ast::{Line, Statement}};

/// Represents a sequence of statements and associated metadata (line numbers)
pub struct Program
{
    /// The list of instructions in order.
    instructions: Vec<Rc<Line>>,
    /// "Saved" or "bookmarked" lines with a reference to their stored location in [instructions]. 
    numbered_lines: HashMap<usize, Rc<Line>>,
}

impl Program
{
    pub fn new() -> Self
    {
        Self
        {
            instructions: Vec::new(),
            numbered_lines: HashMap::new(),
        }
    }

    pub fn add_line(&mut self, line: Line) -> Result<()>
    {
        let num = line.line_number();
        // We use Rc so we can share a reference to the line between both instructions and
        // numbered_lines. You can't have a reference to a sibling member in normal Rust.
        let rc = Rc::new(line);
        self.instructions.push(rc.clone());
        // If we have a line number, we add it to our saved lines.
        if let Some(num) = num
        {
            // TODO: do we want to fail here?
            self.numbered_lines.insert(num, rc);
        }
        Ok(())
    }
}

pub struct Parser();

impl Parser
{
    pub fn parse<T: IntoIterator<Item = Token>>(token_stream: T) -> Result<Program>
    {
        let mut program = Program::new();
        let mut token_stream = token_stream.into_iter();
        let mut token_stream_peek = token_stream.peekable();
        loop
        {
            let next_token = token_stream_peek.peek();
            if next_token.is_none()
            {
                // We're done!
                break;
            }

            let line = match next_token.unwrap()
            {
                // If we have a number, 
                Token::Number(num) => 
                {
                    // Advance the underlying iterator bc we've handled the peeked token which is a
                    // number.
                    _ = token_stream_peek.next();
                    Line::new(Some(*num), Self::parse_statement(&mut token_stream)?)
                }
                token => Line::new(None, Self::parse_statement(&mut token_stream)?)
            };

            program.add_line(line);
        }

        Ok(program)
    }

    fn parse_statement<T: IntoIterator<Item = Token>>(token_stream: &mut T) -> Result<Statement>
    {
        let token = token_stream.into_iter().next();
        if token.is_none()
        {
            anyhow!("No token found!")?;
        }

        Ok(match token.unwrap()
        {
            Token::Keyword(keyword) => match keyword 
            {
                Keyword::Print => todo!(),
                Keyword::If => todo!(),
                Keyword::Then => todo!(),
                Keyword::Goto => todo!(),
                Keyword::Input => todo!(),
                Keyword::Let => todo!(),
                Keyword::GoSub => todo!(),
                Keyword::Return => Statement::Return,
                Keyword::Clear => Statement::Clear,
                Keyword::List => Statement::List,
                Keyword::Run => Statement::Run,
                Keyword::End => Statement::End,
            },
            otherwise => anyhow!(format!("Expected a keyword, found {:?}", otherwise))?,
        })
    }
}

