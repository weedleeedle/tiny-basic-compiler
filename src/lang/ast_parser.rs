//! Parses a [GrammarTree] into an [AST]

use anyhow::anyhow;
use anyhow::bail;

use crate::grammar::ParseGrammarTree;
use crate::grammar::GrammarTree;
use crate::lang::Token;

use super::ast::*;

impl ParseGrammarTree for RelOpSymbol
{
    type Lang = Token;

    fn parse(from: GrammarTree<Self::Lang>) -> anyhow::Result<Self>
        where Self: Sized {
            match from
            {
                GrammarTree::Leaf(_) => bail!("Expected a tree node, got a leaf node"),
                GrammarTree::Node(node) => {
                    let iter = node.children().into_iter();

                    let symbols: anyhow::Result<Vec<Token>> = iter 
                        .map(|x| match *x
                        {
                            // Retrieve the inner symbol
                            GrammarTree::Leaf(token) => Ok(token),
                            _ => Err(anyhow!("Expected a leaf node, got a tree node!")),
                        }).collect();

                    // Return error early
                    let symbols = symbols?;
                    Ok(RelOpSymbol::try_from(symbols.as_slice())?)
                }
            }
    }
}

#[cfg(test)]
mod tests
{
    use crate::grammar::{GrammarBuilder, GrammarNodeData};

    use super::*;

    #[test]
    fn test_relopsymbol()
    {
        // Construct language
        let mut grammar_builder = GrammarBuilder::new();

        let rel_op_symbol = grammar_builder.id();

        // Matches <=
        let leq_rule = Rule::new(rel_op_symbol)
            .add_terminating_symbol(|x| x == Token::Symbol(Symbol::LessThanSign))
            .add_terminating_symbol(|x| x == Token::Symbol(Symbol::Equals));

        // Matches >=
        let geq_rule = Rule::new(rel_op_symbol)
            .add_terminating_symbol(|x| x == Token::Symbol(Symbol::GreaterThanSign))
            .add_terminating_symbol(|x| x == Token::Symbol(Symbol::Equals));

        let grammar = 
            grammar_buider.add_rule(leq_rule)
                      .add_rule(geq_rule)
                      .build();

    }
}

