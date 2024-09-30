//! Marker logic.
use super::Parser;
use crate::{parser::event::Event, syntax::SyntaxKind};
use drop_bomb::DropBomb;

pub struct Marker {
    pos: usize,
    bomb: DropBomb,
}

impl Marker {
    pub fn new(pos: usize) -> Self {
        Self {
            pos,
            bomb: DropBomb::new("Markers _must_ be completed before being dropped!"),
        }
    }

    pub(crate) fn complete(mut self, p: &mut Parser, kind: SyntaxKind) -> CompletedMarker {
        self.bomb.defuse();

        // let event_at_pos = &mut p.events[self.pos];
        let event_at_pos = p.event_at_mut(self.pos);
        assert_eq!(*event_at_pos, Event::Placeholder);

        *event_at_pos = Event::Startnode {
            kind,
            forward_parent: None,
        };

        p.emit_event(Event::FinishNode);

        CompletedMarker { pos: self.pos }
    }
}

pub struct CompletedMarker {
    pos: usize,
}

impl CompletedMarker {
    pub fn precede(self, p: &mut Parser) -> Marker {
        let new_m = p.start();

        if let Event::Startnode {
            ref mut forward_parent,
            ..
        } = p.event_at_mut(self.pos)
        {
            *forward_parent = Some(new_m.pos - self.pos);
        } else {
            unreachable!()
        }

        new_m
    }
}
