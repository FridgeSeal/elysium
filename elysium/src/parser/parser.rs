//! Core parsing logic.

use super::{event, grammar, marker, sink, source};
use crate::{
    lexer::{Lexer, Token, TokenKind},
    syntax::SyntaxKind,
    Parse,
};
use event::Event;
use marker::Marker;
use std::mem;

use sink::Sink;
use source::Source;

mod parse_error;
pub use parse_error::ParseError;

const RECOVERY_SET: [TokenKind; 1] = [TokenKind::LetKw];

/// Parses an input∏ string into a full AST representation.
pub fn parse(input: &str) -> Parse {
    let tokens: Vec<_> = Lexer::new(input).collect();
    let source = Source::new(&tokens);
    let parser = Parser::new(source);
    let events = parser.parse();
    let sink = Sink::new(&tokens, events);

    sink.finish()

    // Parse {
    //     green_node: sink.finish(),
    // }
}

pub struct Parser<'t, 'input> {
    source: Source<'t, 'input>,
    events: Vec<Event>,
    expected_kinds: Vec<TokenKind>,
}

impl<'t, 'input> Parser<'t, 'input> {
    pub const fn new(source: Source<'t, 'input>) -> Self {
        Self {
            source,
            events: Vec::new(),
            expected_kinds: Vec::new(),
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

    fn peek(&mut self) -> Option<TokenKind> {
        self.source.peek_kind() // lazq qgrgcargo depgraph --all-deps | dot -Tsvg > graph.svg.map(|x| x.into())
    }

    pub(crate) fn bump(&mut self) {
        self.expected_kinds.clear();
        self.source.next_token().unwrap();
        self.events.push(Event::AddToken);
    }

    pub(crate) fn at(&mut self, kind: TokenKind) -> bool {
        self.expected_kinds.push(kind);
        self.peek() == Some(kind)
    }

    pub(crate) fn expect(&mut self, kind: TokenKind) {
        if self.at(kind) {
            self.bump();
        } else {
            self.error();
        }
    }

    pub(crate) fn error(&mut self) {
        let current_token = self.source.peek_token();

        let (found, range) = if let Some(Token { kind, range, .. }) = current_token {
            (Some((*kind).into()), *range)
        } else {
            (
                None,
                self.source
                    .last_token_range()
                    .expect("Something has gone violently wrong if this was empty."),
            )
        };

        self.events.push(Event::Error(ParseError {
            expected: mem::take(&mut self.expected_kinds),
            found,
            range,
        }));

        if !self.at_set(&RECOVERY_SET) && !self.at_end() {
            let m = self.start();
            self.bump();
            m.complete(self, SyntaxKind::Error);
        }
    }

    fn at_set(&mut self, set: &[TokenKind]) -> bool {
        self.peek().map_or(false, |k| set.contains(&k))
    }

    pub fn at_end(&mut self) -> bool {
        self.peek().is_none()
    }
}
