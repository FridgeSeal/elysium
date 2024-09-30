mod expr;

// use crate::parser::marker::CompletedMarker;
use super::CompletedMarker;
use crate::parser::Parser;
use crate::syntax::SyntaxKind;

pub fn root(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    expr::expr(p);
    m.complete(p, SyntaxKind::Root)
}
