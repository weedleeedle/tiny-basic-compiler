//! This module is used to implement algorithms and infrastructure for constructing formal grammars
//! For any arbitrary language and converting it into a generic [GrammarTree] structure that 
//! can be converted into whatever your language IR is (AST or whatever).

mod rule;
mod grammar;

pub use rule::*;
pub use grammar::Grammar;
pub use grammar::GrammarBuilder;

/// An abstract tree representing the results from parsing a number of [Rule]s.
pub enum GrammarTree<L>
{
    Leaf(L),
    Node(GrammarNodeData<L>),
}

/// Data contained in a non-leaf [GrammarTree] node.
pub struct GrammarNodeData<L>
{
    /// The ID of the non-terminating symbol that makes up this rule.
    symbol: Id,
    /// A [GrammarTree] node can have an arbitrary number of children.
    children: Vec<Box<GrammarTree<L>>>,
}

