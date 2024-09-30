//! Core functionality for parser.

use rowan::GreenNode;
use syntax::SyntaxNode;
mod lexer;
mod parser;
mod syntax;

pub use parser::parse;

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
    use expect_test::{expect, Expect};

    use crate::parser::parse;

    #[allow(clippy::needless_pass_by_value)]
    fn check(input: &str, expected_tree: Expect) {
        let parse = parse(input);
        expected_tree.assert_eq(&parse.debug_tree());
    }

    #[test]
    fn parse_nothing() {
        check("", expect![[r#"Root@0..0"#]]);
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
