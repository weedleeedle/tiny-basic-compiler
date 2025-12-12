//! Defines the structures for creating formal grammar substitution rules.
//!
//! A rule consists of a (single?) non-terminating symbol mapped to
//! One or more terminal and non-terminating symbols.

use id::IdGenerator;
use id::Id;

pub mod id;

/// The generic parameter `L` is the type of the langauge we are parser.
/// This is probably going to be something like `L::is_keyword()` for
type TokenRecognizer<'a, L> = &'a dyn FnOnce(&L) -> bool;

/// Symbols can be either terminating or non-terminating symbols.
///
/// The generic parameter `L` is the type of the langauge we are parsing.
enum Symbol<'a, L>
{
    Terminating(TokenRecognizer<'a, L>),
    Nonterminating(Id)
}

/// A rule represents a formal grammar expression of some non-terminating symbol to one or more
/// terminating and non-terminating symbols.
///
/// L is the type of the language  we are parsing.
pub struct Rule<'a, L>
{
    // Left-hand input symbol
    input_symbol: Id,
    // Right-hand symbols to replace it with.
    replacement_symbols: Vec<Symbol<'a, L>>
}

impl<'a, L> Rule<'a, L>
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

    pub fn add_terminating_symbol(mut self, terminating_symbol_recognizer: TokenRecognizer<'a, L>) -> Self
    {
        self.replacement_symbols.push(Symbol::Terminating(terminating_symbol_recognizer));
        self
    }
}

pub struct GrammarBuilder<'a, L>
{
    id_generator: IdGenerator,
    starting_rule: Option<Rule<'a, L>>,
    rules: Vec<Rule<'a, L>>
}

impl<'a, L> GrammarBuilder<'a, L>
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
    pub fn add_rule(mut self, rule: Rule<'a, L>) -> Self
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
    pub fn build(self) -> Option<Grammar<'a, L>>
    {
        Some(Grammar
        {
            id_generator: self.id_generator,
            default_rule: self.starting_rule?,
            rules: self.rules,
        })
    }
}

/// A completed set of rules defining a certain formal grammar.
///
/// L is the type of the language we are parsing.
pub struct Grammar<'a, L>
{
    id_generator: IdGenerator,
    default_rule: Rule<'a, L>,
    rules: Vec<Rule<'a, L>>
}

#[cfg(test)]
mod tests
{
    struct MockLang();
    
    impl MockLang
    {
        pub fn test_func(&self) -> bool
        {
            true
        }
    }

    use super::*;

    #[test]
    fn test_create_symbol()
    {
        let mut grammar = GrammarBuilder::<MockLang>::new();
        let s = grammar.id();
        let t = grammar.id();
        assert_ne!(s, t);
    }

    #[test]
    fn test_create_rule()
    {
        let mut grammar = GrammarBuilder::<MockLang>::new();
        let s = grammar.id();

        let rule = Rule::new(s)
            .add_nonterminating_symbol(s)
            .add_terminating_symbol(&MockLang::test_func);

        grammar.add_rule(rule);
    }
}
