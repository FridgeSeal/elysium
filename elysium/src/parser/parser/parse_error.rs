//! Parser errors.

use crate::lexer::TokenKind;
use rowan::TextRange;
use std::fmt;

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct ParseError {
    pub(super) expected: Vec<TokenKind>,
    pub(super) found: Option<TokenKind>,
    pub(super) range: TextRange,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "error at {}..{}: expected ",
            u32::from(self.range.start()),
            u32::from(self.range.end()),
        )?;

        let num_expected = self.expected.len();
        let first = |idx| idx == 0;
        let last = |idx| idx == num_expected - 1;

        for (idx, expected_kind) in self.expected.iter().enumerate() {
            if first(idx) {
                write!(f, "{expected_kind}")?;
            } else if last(idx) {
                write!(f, " or {expected_kind}")?;
            } else {
                write!(f, ", {expected_kind}")?;
            }
        }

        if let Some(found) = self.found {
            write!(f, ", but found {found}")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use super::*;

    fn check(expected: Vec<TokenKind>, found: Option<TokenKind>, range: Range<u32>, output: &str) {
        let error = ParseError {
            expected,
            found,
            range: {
                let start = range.start.into();
                let end = range.end.into();
                TextRange::new(start, end)
            },
        };

        assert_eq!(format!("{error}"), output)
    }

    #[test]
    fn one_expect_did_find() {
        check(
            vec![TokenKind::Equals],
            Some(TokenKind::Ident),
            10..20,
            "error at 10..20: expected '=', but found identifier",
        );
    }

    #[test]
    fn one_expected_did_not_find() {
        check(
            vec![TokenKind::RParen],
            None,
            5..6,
            "error at 5..6: expected ')'",
        )
    }

    #[test]
    fn multiple_expected_did_find() {
        check(
            vec![
                TokenKind::Number,
                TokenKind::Ident,
                TokenKind::Minus,
                TokenKind::LParen,
            ],
            Some(TokenKind::LetKw),
            100..105,
            "error at 100..105: expected number-literal, identifier, '-' or '(', but found 'let'",
        );
    }

    #[test]
    fn two_expected_did_find() {
        check(
            vec![TokenKind::Plus, TokenKind::Minus],
            Some(TokenKind::Equals),
            0..1,
            "error at 0..1: expected '+' or '-', but found '='",
        );
    }
}
