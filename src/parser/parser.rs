//! This module defines the actual parser implementation that produces an AST from a stream of
//! tokens.

use std::{collections::HashMap, slice::Iter};

use anyhow::Result;
use derive_more::derive;

use crate::{lexer::{Keyword, Token}, parser::ast::{Line, Statement}};

/// Represents a sequence of statements and associated metadata (line numbers)
#[derive(New)]
pub struct Program
{
    /// The list of instructions in order.
    instructions: Vec<Line>,
    /// "Saved" or "bookmarked" lines with a reference to their stored location in [instructions]. 
    numbered_lines: HashMap<int, &Line>,
}

impl Program
{
    pub fn add_line(&mut self, line: Line) -> Result<()>
    {
        let num = line.line_number();
        self.instructions.push(line);
        // If we have a line number, we add it to our saved lines.
        if let Some(num) = num
        {
            let line_ref = self.instructions.last().unwrap();
            // TODO: do we want to fail here?
            self.numbered_lines.insert(num, line_ref);
        }
    }
}

pub struct Parser();

impl Parser
{
    pub fn parse(token_stream: impl IntoIterator<Item = Token>) -> Result<Program>
    {
        let mut program = Program::new();
        let mut token_stream = token_stream.into_iter();
        loop
        {
            let token = token_stream.next();
            if token.is_none()
            {
                // We're done!
                break;
            }

            let line = match token
            {
                // If we have a number, 
                Token::Number(num) => 
                {
                    Line::new(Some(num), parse_statement(&token_stream)?)
                }
                _ => Line::new(None, Self::parse_statement(token.iter().chain(&token_stream))?)
            }

            program.add_line(line);
        }
    }

    fn parse_statement(token_stream: &mut Iterator<Item = Token>) -> Result<Statement>
    {
        let token = token_stream.next();
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

