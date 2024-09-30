//! Core parsing logic.

use crate::{lexer::Lexer, syntax::SyntaxKind, Parse};
use event::Event;
use marker::Marker;
use sink::Sink;
use source::Source;

use super::{event, grammar, marker, sink, source};

const RECOVERY_SET: [SyntaxKind; 1] = [SyntaxKind::LetKw];

/// Parses an input string into a full AST representation.
pub fn parse(input: &str) -> Parse {
    let tokens: Vec<_> = Lexer::new(input).collect();
    let source = Source::new(&tokens);
    let parser = Parser::new(source);
    let events = parser.parse();
    let sink = Sink::new(&tokens, events);

    Parse {
        green_node: sink.finish(),
    }
}

pub struct Parser<'t, 'input> {
    source: Source<'t, 'input>,
    events: Vec<Event>,
}

impl<'t, 'input> Parser<'t, 'input> {
    pub const fn new(source: Source<'t, 'input>) -> Self {
        Self {
            source,
            events: Vec::new(),
        }
    }

    pub(crate) fn emit_event(&mut self, e: Event) {
        self.events.push(e);
    }

    pub(crate) fn event_at_mut(&mut self, pos: usize) -> &mut Event {
        &mut self.events[pos]
    }
    pub(crate) fn start(&mut self) -> Marker {
        let pos = self.events.len();
        self.events.push(Event::Placeholder);
        Marker::new(pos)
    }

    pub(crate) fn parse(mut self) -> Vec<Event> {
        grammar::root(&mut self);
        self.events
    }

    pub(crate) fn peek(&mut self) -> Option<SyntaxKind> {
        self.source.peek_kind() // lazq qgrgcargo depgraph --all-deps | dot -Tsvg > graph.svg.map(|x| x.into())
    }

    pub(crate) fn bump(&mut self) {
        self.source.next_token().unwrap();
        self.events.push(Event::AddToken);
    }

    pub(crate) fn at(&mut self, kind: SyntaxKind) -> bool {
        self.peek() == Some(kind)
    }

    pub(crate) fn expect(&mut self, kind: SyntaxKind) {
        if self.at(kind) {
            self.bump();
        } else {
            self.error();
        }
    }

    pub(crate) fn error(&mut self) {
        if !self.at_set(&RECOVERY_SET) {
            self.bump();
        }
    }

    fn at_set(&mut self, set: &[SyntaxKind]) -> bool {
        self.peek().map_or(false, |k| set.contains(&k))
    }
}
