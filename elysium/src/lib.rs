//! Core functionality for parser.

use rowan::GreenNode;
use syntax::{SyntaxNode, SyntaxToken};
mod ast;
mod lexer;
mod parser;
mod syntax;

pub use parser::parse;

/// AST like structure.
pub struct Parse {
    green_node: GreenNode,
    errors: Vec<parser::ParseError>,
}

impl Parse {
    /// Produces the debug-friend representation of the AST.
    pub fn debug_tree(&self) -> String {
        let mut s = String::new();

        let syntax_node = SyntaxNode::new_root(self.green_node.clone());
        let tree = format!("{syntax_node:#?}");

        s.push_str(&tree[0..tree.len() - 1]);

        for error in &self.errors {
            s.push_str(&format!("\n{error}"));
        }

        s
    }
}

#[cfg(test)]
mod tests {
    use expect_test::{expect, Expect};

    use crate::parser::parse;

    #[allow(clippy::needless_pass_by_value)]
    pub fn check(input: &str, expected_tree: Expect) {
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
