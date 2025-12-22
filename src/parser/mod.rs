//! Takes a stream of tokens and produces an AST from it.

pub mod rule;

use std::iter::{self, Chain, Once};
use std::slice::Iter;

use crate::parser::rule::id::{Id, IdGenerator};
use crate::parser::rule::{Rule, SymbolInstance};

/// The resulting tree after a [Grammar] parses a sequence of tokens in `L` language.
#[derive(Debug)]
pub enum ParsedGrammarTree<L>
{
    Leaf(L),
    Node(ParseTreeNodeData<L>),
}

impl<'a, L> From<&'a ParsedGrammarTree<L>> for SymbolInstance<'a, L>
{
    fn from(value: &'a ParsedGrammarTree<L>) -> Self {
        match value
        {
            ParsedGrammarTree::Leaf(l) => SymbolInstance::Terminating(l),
            ParsedGrammarTree::Node(node) => SymbolInstance::Nonterminating(node.symbol),
        }
    }
}

#[derive(Debug)]
pub struct ParseTreeNodeData<L>
{
    // the ID of the non-terminating symbol.
    symbol: Id,
    // A [ParsedGrammarTree] node can have an arbitrary number of children.
    children: Vec<Box<ParsedGrammarTree<L>>>,
}

pub trait ParseEngine
{
    type InputLang;

    fn parse_input(&self, input_iter: impl Iterator<Item = Self::InputLang>) -> ParsedGrammarTree<Self::InputLang>;
}

pub trait FromParseTree 
{
    type InputLang;

    /// Parses a [ParsedGrammarTree] over an input lang and returns a new [Self] if the input tree
    /// matches the expected tree shape. Otherwise returns [None].
    fn from_parse_tree(input: ParsedGrammarTree<Self::InputLang>) -> Option<Self> where Self: Sized;
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

impl<L> Grammar<'_, L>
{
    // Gets an iterator over all the rules.
    fn rules(&self) -> Chain<Once<&Rule<'_, L>>, Iter<'_, Rule<'_, L>>>
    {
        iter::once(&self.default_rule)
            .chain(
                self.rules.iter()
            )
    }

    /// Can return none if like the input stream is empty or something?
    pub fn parse(&self, input: impl IntoIterator<Item = L>) -> Option<ParsedGrammarTree<L>>
    {
        // Initialize state.
        let input_stream = input.into_iter();
        let mut input_stack: Vec<ParsedGrammarTree<L>> = Vec::new();

        // iterate over the entire input stream.
        for next_symbol in input_stream
        {
            let mut input_symbols = convert_input_stack_to_symbol_instances(&input_stack);
            input_symbols.push(SymbolInstance::Terminating(&next_symbol));
            let mut reduce_found = false;
            // Attempt to reduce the input stack by combining one or more symbols into a
            // non-terminating symbol according to one of our rules.
            //
            // We also probably need more intelligent rules for how to check against rules...
            for rule in self.rules()
            {
                if rule.matches(&input_symbols)
                {
                    let mut children: Vec<Box<ParsedGrammarTree<L>>> = Vec::new();
                    // Pop the last N-1 symbols from the stack and replace them with the input
                    // symbol
                    for _ in 1..input_symbols.len()
                    {
                        match input_stack.pop()
                        {
                            Some(node) => children.push(Box::new(node)),
                            // Idk if this is actually unreachable or not?
                            None => unreachable!()
                        }
                    }

                    // Create a new node and push it back onto the stack.
                    let new_parse_tree_node = ParseTreeNodeData::<L>
                    {
                        symbol: rule.input_symbol(),
                        children,
                    };

                    input_stack.push(ParsedGrammarTree::Node(new_parse_tree_node));
                    reduce_found = true;

                    break;
                }
            }

            // If we weren't able to reduce the tree, just add the next token to the stack as is
            // ("shift")
            if !reduce_found
            {
                input_stack.push(ParsedGrammarTree::Leaf(next_symbol));
            }
        }

        input_stack.pop()
    }
}

fn convert_input_stack_to_symbol_instances<'a, L>(input_stack: &'a [ParsedGrammarTree<L>]) -> Vec<SymbolInstance<'a, L>>
{
    input_stack.iter().map(|x| x.into()).collect()
}

#[cfg(test)]
mod tests
{
    use super::*;
    
    #[derive(Debug)]
    enum MockLangToken
    {
        A,
        B,
    }

    impl MockLangToken
    {
        pub fn is_a(&self) -> bool
        {
            match self
            {
                Self::A => true,
                Self::B => false,
            }
        }

        pub fn is_b(&self) -> bool
        {
            match self
            {
                Self::A => false,
                Self::B => true,
            }
        }
    }


    #[test]
    fn test_one_rule_grammar()
    {
        let mut grammar_builder = GrammarBuilder::<MockLangToken>::new();

        let symbol = grammar_builder.id();

        let rule = Rule::new(symbol)
            .add_terminating_symbol(&MockLangToken::is_a)
            .add_terminating_symbol(&MockLangToken::is_a);

        let grammar_builder = grammar_builder.add_rule(rule);

        let grammar = grammar_builder.build().unwrap();
        let input = vec![
            MockLangToken::A,
            MockLangToken::A,
        ];

        let result = grammar.parse(input);
        let result = result.unwrap();
        match result
        {
            ParsedGrammarTree::Leaf(_) => panic!("Expected Node, got Leaf!"),
            ParsedGrammarTree::Node(node) => {
                assert_eq!(node.symbol, symbol);
                for node in node.children
                {
                    match *node
                    {
                        ParsedGrammarTree::Node(_) => panic!("Expected Leaf, got Node!"),
                        ParsedGrammarTree::Leaf(l) => assert!(MockLangToken::is_a(&l)),
                    }
                }
            },
        }
    }
}
