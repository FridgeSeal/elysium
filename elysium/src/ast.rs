//! Functionality for the AST built on top of the Rowan CST.

use crate::{syntax::SyntaxToken, SyntaxNode};

pub struct VariableDef(SyntaxNode);

impl VariableDef {
    pub fn name(&self) -> Option<SyntaxToken> {
        unimplemented!()
    }

    pub fn value(&self) -> Option<SyntaxNode> {
        unimplemented!()
    }
}
