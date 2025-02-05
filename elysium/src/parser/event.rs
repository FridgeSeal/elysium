use crate::syntax::SyntaxKind;

use super::parser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Startnode {
        kind: SyntaxKind,
        forward_parent: Option<usize>,
    },

    AddToken,
    FinishNode,
    Error(parser::ParseError),
    Placeholder,
}
