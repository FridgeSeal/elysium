mod event;
mod expr;
mod sink;
mod source;

use crate::{
    lexer::{Lexeme, Lexer, SyntaxKind},
    syntax::SyntaxNode,
};
use event::Event;
use expr::expr;
use rowan::GreenNode;
use sink::Sink;
use source::Source;

struct Parser<'l, 'input> {
    source: Source<'l, 'input>,
    events: Vec<Event>,
}

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
    pub fn new(lexemes: &'l [Lexeme<'input>]) -> Self {
        Self {
            source: Source::new(lexemes),
            events: Vec::new(),
        }
    }

    fn parse(mut self) -> Vec<Event> {
        self.start_node(SyntaxKind::Root);
        expr(&mut self);
        self.finish_node();

        self.events
    }

    fn peek(&mut self) -> Option<SyntaxKind> {
        self.source.peek_kind()
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        self.events.push(Event::Startnode { kind });
        // self.builder.start_node(ElysiumLanguage::kind_to_raw(kind));
    }

    fn finish_node(&mut self) {
        self.events.push(Event::FinishNode);
    }

    fn bump(&mut self) {
        let Lexeme { kind, text } = self.source.next_lexeme().unwrap();
        self.events.push(Event::AddToken {
            kind: *kind,
            text: text.to_string(),
        });
    }

    fn start_node_at(&mut self, checkpoint: usize, kind: SyntaxKind) {
        self.events.push(Event::StartNodeAt { kind, checkpoint });
    }

    fn checkpoint(&self) -> usize {
        self.events.len()
    }
}

pub struct Parse {
    green_node: GreenNode,
}

impl Parse {
    pub fn debug_tree(&self) -> String {
        let syntax_node = SyntaxNode::new_root(self.green_node.clone());
        let formatted = format!("{:#?}", syntax_node);

        formatted[0..formatted.len() - 1].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use expect_test::{expect, Expect};

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
}
