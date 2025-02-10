use core::fmt;

use super::Literal;
use crate::syntax::SyntaxNode;
use rowan::TextRange;

pub fn validate(node: &SyntaxNode) -> Vec<ValidationError> {
    let mut errors = Vec::new();

    for node in node.descendants() {
        if let Some(literal) = Literal::cast(node) {
            validate_literal(&mut errors, literal);
        }
    }

    errors
}

fn validate_literal(errors: &mut Vec<ValidationError>, literal: Literal) {
    if literal.parse().is_none() {
        errors.push(ValidationError {
            kind: ValidationErrorKind::NumberLiteralTooLarge,
            range: literal.inner().first_token().unwrap().text_range(),
        });
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ValidationError {
    kind: ValidationErrorKind,
    range: TextRange,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "error at {}..{}: {}",
            u32::from(self.range.start()),
            u32::from(self.range.end()),
            self.kind
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ValidationErrorKind {
    NumberLiteralTooLarge,
}

impl fmt::Display for ValidationErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NumberLiteralTooLarge => write!(
                f,
                "Number literal is larger than supported integer maximum value of {}",
                u64::MAX
            ),
        }
    }
}

// probably dead code, but it's here if it gets referenced later.
// fn validate_expr(errors: &mut Vec<ValidationError>, expr: Expr) {
//     match expr {
//         Expr::BinaryExpr(binary_expr) => {
//             if let Some(e) = binary_expr.lhs() {
//                 validate_expr(errors, e);
//             }
//             if let Some(e) = binary_expr.rhs() {
//                 validate_expr(errors, e);
//             }
//         }
//         Expr::Literal(literal) => {
//             if literal.parse().is_none() {
//                 errors.push(ValidationError {
//                     message: format!(
//                         "Number literal is larger than supported integer maximum value {}",
//                         u64::MAX
//                     ),
//                     range: literal.0.first_token().unwrap().text_range(),
//                 });
//             }
//         }
//         Expr::ParenExpr(paren_expr) => {
//             if let Some(e) = paren_expr.expr() {
//                 validate_expr(errors, e);
//             }
//         }
//         Expr::UnaryExpr(unary_expr) => {
//             if let Some(e) = unary_expr.expr() {
//                 validate_expr(errors, e);
//             }
//         }
//         Expr::VariableRef(_) => {}
//     }
// }

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use crate::parse;

    use super::*;

    fn check(input: &str, expected_errors: &[(ValidationErrorKind, Range<u32>)]) {
        let parse = parse(input);

        let expected_errors: Vec<_> = expected_errors
            .iter()
            .map(|(kind, range)| ValidationError {
                kind: *kind,
                range: {
                    let start = range.start.into();
                    let end = range.end.into();
                    TextRange::new(start, end)
                },
            })
            .collect();

        assert_eq!(validate(&parse.syntax()), expected_errors);
    }

    #[test]
    fn validate_ok_literal() {
        check("123", &[]);
    }

    #[test]
    fn validate_too_large_literal() {
        check(
            "99999999999999999999",
            &[(ValidationErrorKind::NumberLiteralTooLarge, (0..20))],
        );
    }
}
