use crate::lexer::SyntaxKind;
// use rowan::SmolStr;

#[derive(Debug, Clone)]
pub(super) enum Event {
    Startnode { kind: SyntaxKind },
    StartNodeAt { kind: SyntaxKind, checkpoint: usize },
    AddToken { kind: SyntaxKind, text: String },
    FinishNode,
}
