mod expr;
mod stmt;
use super::CompletedMarker;
use crate::parser::Parser;
use crate::syntax::SyntaxKind;

pub fn root(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    while !p.at_end() {
        stmt::stmt(p);
    }

    m.complete(p, SyntaxKind::Root)
}
