//! This module is used to implement algorithms and infrastructure for constructing formal grammars
//! For any arbitrary language and converting it into a generic [GrammarTree] structure that 
//! can be converted into whatever your language IR is (AST or whatever).

mod rule;
mod grammar;

use getset::CopyGetters;
use getset::Getters;
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
#[derive(CopyGetters)]
pub struct GrammarNodeData<L>
{
    /// The ID of the non-terminating symbol that makes up this rule.
    #[getset(get_copy = "pub")]
    symbol: Id,
    /// A [GrammarTree] node can have an arbitrary number of children.
    children: Vec<Box<GrammarTree<L>>>,
}

impl<L> GrammarNodeData<L>
{
    pub fn children(self) -> Vec<Box<GrammarTree<L>>>
    {
        self.children
    }
}

pub trait ParseGrammarTree
{
    type Lang;

    fn parse(from: GrammarTree<Self::Lang>) -> anyhow::Result<Self>
        where Self: Sized;
}
