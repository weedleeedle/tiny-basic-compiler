//! Defines the [Grammar] and [GrammarBuilder] types.

use std::iter;
use std::iter::Chain;
use std::iter::Once;
use std::slice::Iter;

use crate::grammar::GrammarNodeData;
use crate::grammar::GrammarTree;
use crate::grammar::Id;
use crate::grammar::IdGenerator;
use crate::grammar::Rule;

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
    pub fn parse(&self, input: impl IntoIterator<Item = L>) -> Option<GrammarTree<L>>
    {
        // Initialize state.
        let input_stream = input.into_iter();
        let mut input_stack: Vec<GrammarTree<L>> = Vec::new();

        // iterate over the entire input stream.
        for next_symbol in input_stream
        {
            //let mut input_symbols = convert_input_stack_to_symbol_instances(&input_stack);
            //input_symbols.push(SymbolInstance::Terminating(&next_symbol));

            // We start by pushing the new symbol onto the stack.
            input_stack.push(GrammarTree::Leaf(next_symbol));

            let mut reduce_found = false;
            // Attempt to reduce the input stack by combining one or more symbols into a
            // non-terminating symbol according to one of our rules.
            //
            // We attempt to greedily match as many symbols as possible.
            // For each failed attempt, we try to match one last symbol to a rule until we 
            // finally find one that works.
            for i in 0..input_stack.len()
            {
                // Match the last `i` symbols.
                let input_stack_slice = &input_stack[i..];

                // Try to match our slice of symbols against any one of our rules.
                for rule in self.rules()
                {
                    // If we find a rule that matches,
                    // We pull the matching symbols off the stack and replace it with the
                    // non-terminating symbol.
                    //
                    // i.e if we have a rule that says A -> ab
                    // and we find "ab", we replace it with A.
                    if rule.matches(input_stack_slice)
                    {
                        let mut children: Vec<Box<GrammarTree<L>>> = Vec::new();
                        // Pop the last N-1 symbols from the stack and replace them with the input
                        // symbol
                        for _ in 0..input_stack_slice.len()
                        {
                            match input_stack.pop()
                            {
                                Some(node) => children.push(Box::new(node)),
                                // Idk if this is actually unreachable or not?
                                None => unreachable!()
                            }
                        }

                        // Create a new node and push it back onto the stack.
                        let new_parse_tree_node = GrammarNodeData::<L>
                        {
                            symbol: rule.input_symbol(),
                            children,
                        };

                        input_stack.push(GrammarTree::Node(new_parse_tree_node));
                        reduce_found = true;

                        break;
                    }
                }

                // Abort searching through the stack if we found a valid reduction.
                if reduce_found
                {
                    break;
                }
            }
        }

        // Remove the very last symbol we found.
        // We may want to make sure that this is the ONLY symbol on the tree? Idk.
        input_stack.pop()
    }
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
            GrammarTree::Leaf(_) => panic!("Expected Node, got Leaf!"),
            GrammarTree::Node(node) => {
                assert_eq!(node.symbol, symbol);
                for node in node.children
                {
                    match *node
                    {
                        GrammarTree::Node(_) => panic!("Expected Leaf, got Node!"),
                        GrammarTree::Leaf(l) => assert!(MockLangToken::is_a(&l)),
                    }
                }
            },
        }
    }
}
