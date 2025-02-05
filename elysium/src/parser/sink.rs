use super::{event::Event, parser::ParseError};
use crate::{lexer::Token, syntax::ElysiumLanguage, Parse};
use rowan::{GreenNodeBuilder, Language};
use std::mem;

pub struct Sink<'l, 'input> {
    builder: GreenNodeBuilder<'static>,
    tokens: &'l [Token<'input>],
    cursor: usize,
    events: Vec<Event>,
    errors: Vec<ParseError>,
}

impl<'l, 'input> Sink<'l, 'input> {
    pub fn new(tokens: &'l [Token<'input>], events: Vec<Event>) -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
            tokens,
            cursor: 0,
            events,
            errors: Vec::new(),
        }
    }

    pub fn finish(mut self) -> Parse {
        for idx in 0..self.events.len() {
            match mem::replace(&mut self.events[idx], Event::Placeholder) {
                Event::Startnode {
                    kind,
                    forward_parent,
                } => {
                    let mut kinds = vec![kind];
                    let mut idx = idx;
                    let mut forward_parent = forward_parent;
                    while let Some(fp) = forward_parent {
                        idx += fp;
                        forward_parent = if let Event::Startnode {
                            kind,
                            forward_parent,
                        } =
                            mem::replace(&mut self.events[idx], Event::Placeholder)
                        {
                            kinds.push(kind);
                            forward_parent
                        } else {
                            unreachable!()
                        }
                    }

                    for kind in kinds.into_iter().rev() {
                        self.builder.start_node(ElysiumLanguage::kind_to_raw(kind));
                    }
                }

                Event::AddToken => self.token(),
                Event::FinishNode => self.builder.finish_node(),
                Event::Error(err) => self.errors.push(err),
                Event::Placeholder => {}
            }

            self.eat_trivia();
        }

        Parse {
            green_node: self.builder.finish(),
            errors: self.errors,
        }
    }

    fn token(&mut self) {
        let Token { kind, text, .. } = self.tokens[self.cursor];
        self.builder
            .token(ElysiumLanguage::kind_to_raw(kind.into()), text);
        self.cursor += 1;
    }

    fn eat_trivia(&mut self) {
        while let Some(token) = self.tokens.get(self.cursor) {
            if !token.kind.is_trivia() {
                break;
            }

            self.token();
        }
    }
}
