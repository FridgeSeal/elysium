use crate::syntax::SyntaxKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Event {
    Startnode {
        kind: SyntaxKind,
        forward_parent: Option<usize>,
    },

    AddToken,
    FinishNode,
    Placeholder,
}
