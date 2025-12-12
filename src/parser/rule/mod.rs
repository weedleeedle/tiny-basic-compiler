//! Defines the structures for creating formal grammar substitution rules.
//!
//! A rule consists of a (single?) non-terminating symbol mapped to
//! One or more terminal and non-terminating symbols.

use id::IdGenerator;
use id::Id;
use crate::lexer::Token;

pub mod id;

type TokenRecognizer<'a> = &'a dyn FnOnce(&Token) -> bool;

/// Symbols can be either terminating or non-terminating symbols.
enum Symbol<'a>
{
    // Okay I'm still brain dead about this but what I want to do is check the type of the symbol
    // So this could be something like passing in the Token::is_keyword() or w/e.
    Terminating(TokenRecognizer<'a>),
    Nonterminating(Id)
}

pub struct Rule<'a>
{
    // Left-hand input symbol
    input_symbol: Id,
    // Right-hand symbols to replace it with.
    replacement_symbols: Vec<Symbol<'a>>
}

impl<'a> Rule<'a>
{
    pub fn new(input_symbol: Id) -> Self
    {
        Self
        {
            input_symbol,
            replacement_symbols: Vec::new()
        }
    }

    pub fn add_nonterminating_symbol(mut self, symbol: Id) -> Self
    {
        self.replacement_symbols.push(Symbol::Nonterminating(symbol));
        self
    }

    pub fn add_terminating_symbol(mut self, terminating_symbol_recognizer: TokenRecognizer<'a>) -> Self
    {
        self.replacement_symbols.push(Symbol::Terminating(terminating_symbol_recognizer));
        self
    }
}

pub struct GrammarBuilder<'a>
{
    id_generator: IdGenerator,
    starting_rule: Option<Rule<'a>>,
    rules: Vec<Rule<'a>>
}

impl<'a> GrammarBuilder<'a>
{
    pub fn new() -> Self
    {
        Self
        {
            id_generator: IdGenerator::new(),
            starting_rule: None,
            rules: Vec::new(),
        }
    }

    pub fn id(&mut self) -> Id
    {
        self.id_generator.id()
    }

    /// Adds a new rule to the grammar. The first rule added is the "default" or first rule. All
    /// other rules are specified later.
    ///
    /// # Arguments
    ///
    /// * `rule` - A rule
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn add_rule(mut self, rule: Rule<'a>) -> Self
    {
        if self.starting_rule.is_none()
        {
            self.starting_rule = Some(rule);
        }
        else
        {
            self.rules.push(rule);
        }
        self
    }

    /// Builds a [Grammar]. A [GrammarBuilder] expects there to be at least one rule specified,
    /// otherwise it returns [None]
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn build(self) -> Option<Grammar<'a>>
    {
        Some(Grammar
        {
            id_generator: self.id_generator,
            default_rule: self.starting_rule?,
            rules: self.rules,
        })
    }
}

pub struct Grammar<'a>
{
    id_generator: IdGenerator,
    default_rule: Rule<'a>,
    rules: Vec<Rule<'a>>
}


#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_create_symbol()
    {
        let mut grammar = GrammarBuilder::new();
        let s = grammar.id();
        let t = grammar.id();
        assert_ne!(s, t);
    }

    #[test]
    fn test_create_rule()
    {
        let mut grammar = GrammarBuilder::new();
        let s = grammar.id();

        let rule = Rule::new(s)
            .add_nonterminating_symbol(s)
            .add_terminating_symbol(&Token::is_keyword);

        grammar.add_rule(rule);
    }
}
