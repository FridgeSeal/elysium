//! Core parsing logic.
mod event;
mod expr;
mod marker;
mod sink;
mod source;

use crate::{
    lexer::{Lexer, Token},
    syntax::{SyntaxKind, SyntaxNode},
};
use event::Event;
use expr::expr;
use marker::Marker;
use rowan::GreenNode;
use sink::Sink;
use source::Source;

struct Parser<'l, 'input> {
    source: Source<'l, 'input>,
    events: Vec<Event>,
}

/// Parses an input string into a full AST representation.
pub fn parse(input: &str) -> Parse {
    let lexemes: Vec<_> = Lexer::new(input).collect();
    let parser = Parser::new(&lexemes);
    let events = parser.parse();
    let sink = Sink::new(&lexemes, events);

    Parse {
        green_node: sink.finish(),
    }
}

impl<'l, 'input> Parser<'l, 'input> {
    pub const fn new(lexemes: &'l [Token<'input>]) -> Self {
        Self {
            source: Source::new(lexemes),
            events: Vec::new(),
        }
    }

    fn start(&mut self) -> Marker {
        let pos = self.events.len();
        self.events.push(Event::Placeholder);
        Marker::new(pos)
    }

    fn parse(mut self) -> Vec<Event> {
        let m = self.start();
        expr(&mut self);
        m.complete(&mut self, SyntaxKind::Root);

        self.events
    }

    fn peek(&mut self) -> Option<SyntaxKind> {
        self.source.peek_kind().map(|x| x.into())
    }

    fn bump(&mut self) {
        self.source.next_token().unwrap();
        self.events.push(Event::AddToken);
    }

    fn at(&mut self, kind: SyntaxKind) -> bool {
        self.peek() == Some(kind)
    }
}

/// AST like structure.
pub struct Parse {
    green_node: GreenNode,
}

impl Parse {
    /// Produces the debug-friend representation of the AST.
    pub fn debug_tree(&self) -> String {
        let syntax_node = SyntaxNode::new_root(self.green_node.clone());
        let formatted = format!("{syntax_node:#?}");

        formatted[0..formatted.len() - 1].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use expect_test::{expect, Expect};

    #[allow(clippy::needless_pass_by_value)]
    fn check(input: &str, expected_tree: Expect) {
        let parse = parse(input);
        expected_tree.assert_eq(&parse.debug_tree());
    }

    #[test]
    fn parse_whitespace() {
        check(
            "   ",
            expect![[r#"
Root@0..3
  Whitespace@0..3 "   ""#]],
        );
    }

    #[test]
    fn parse_comment() {
        check(
            "# hello!",
            expect![[r##"
Root@0..8
  Comment@0..8 "# hello!""##]],
        )
    }
}
