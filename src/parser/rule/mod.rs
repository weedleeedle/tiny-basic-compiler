//! Defines the structures for creating formal grammar substitution rules.
//!
//! A rule consists of a (single?) non-terminating symbol mapped to
//! One or more terminal and non-terminating symbols.
//!

use id::Id;

pub mod id;

/// The generic parameter `L` is the type of the langauge we are parser.
/// This is probably going to be something like `L::is_keyword()` for
type TokenRecognizer<'a, L> = &'a dyn Fn(&L) -> bool;

/// Symbols can be either terminating or non-terminating symbols.
///
/// The generic parameter `L` is the type of the langauge we are parsing.
///
/// Schema means that this type is used in the definition of rules and symbols.
///
/// When we actually want to see if a sequence of tokens match, we use [SymbolInstance] instead.
pub enum SymbolSchema<'a, L>
{
    Terminating(TokenRecognizer<'a, L>),
    Nonterminating(Id)
}


/// An actual instance of a symbol. We can tell if a sequence of [SymbolInstance]s matches a [Rule]
/// by checking it. Basically.
pub enum SymbolInstance<'a, L>
{
    Terminating(&'a L),
    Nonterminating(Id),
}

/// A rule represents a formal grammar expression of some non-terminating symbol to one or more
/// terminating and non-terminating symbols.
///
/// L is the type of the language we are parsing.
pub struct Rule<'a, L>
{
    // Left-hand input symbol
    input_symbol: Id,
    // Right-hand symbols to replace it with.
    replacement_symbols: Vec<SymbolSchema<'a, L>>
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
        self.replacement_symbols.push(SymbolSchema::Nonterminating(symbol));
        self
    }

    pub fn add_terminating_symbol(mut self, terminating_symbol_recognizer: TokenRecognizer<'a, L>) -> Self
    {
        self.replacement_symbols.push(SymbolSchema::Terminating(terminating_symbol_recognizer));
        self
    }

    pub fn matches(&self, rhs: &[SymbolInstance<L>]) -> bool
    {
        if self.replacement_symbols.len() != rhs.len()
        {
            return false;
        }

        for (symbol_schema, symbol_instance) in self.replacement_symbols.iter().zip(rhs)
        {
            // Check to see if the symbols match.
            let symbol_match = match (symbol_schema, symbol_instance)
            {
                (SymbolSchema::Terminating(func), SymbolInstance::Terminating(token)) => func(token),
                (SymbolSchema::Terminating(_), SymbolInstance::Nonterminating(_)) => false,
                (SymbolSchema::Nonterminating(_), SymbolInstance::Terminating(_)) => false,
                (SymbolSchema::Nonterminating(id), SymbolInstance::Nonterminating(id_2)) => id == id_2,
            };

            // If they don't, abort. Otherwise continue.
            if !symbol_match 
            {
                return false;
            }
        }

        return true;
    }

    pub fn input_symbol(&self) -> Id
    {
        self.input_symbol
    }
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

    use crate::parser::GrammarBuilder;

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

    #[test]
    fn test_rule_match()
    {
        let mut grammar = GrammarBuilder::<MockLang>::new();
        let s = grammar.id();

        let rule = Rule::new(s)
            .add_terminating_symbol(&MockLang::test_func)
            .add_terminating_symbol(&MockLang::test_func);

        let input_symbols_wrong = vec![
            SymbolInstance::<MockLang>::Terminating(&MockLang())
        ];

        let input_symbols_right = vec![
            SymbolInstance::<MockLang>::Terminating(&MockLang()),
            SymbolInstance::<MockLang>::Terminating(&MockLang()),
        ];

        assert!(!rule.matches(&input_symbols_wrong));
        assert!(rule.matches(&input_symbols_right));
    }

    #[test]
    fn test_rule_match_with_nonterminating_symbols()
    {
        let mut grammar = GrammarBuilder::<MockLang>::new();
        let s = grammar.id();
        let t = grammar.id();

        let rule = Rule::new(s)
            .add_terminating_symbol(&MockLang::test_func)
            .add_nonterminating_symbol(t);

        let input_symbols = vec![
            SymbolInstance::<MockLang>::Terminating(&MockLang()),
            SymbolInstance::<MockLang>::Nonterminating(t)
        ];

        assert!(rule.matches(&input_symbols));
    }

}
