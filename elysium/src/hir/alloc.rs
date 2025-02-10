// Called "database" in the tutorial.

use la_arena::Arena;

// use crate::arena::Arena;
use super::{BinaryOp, HirExpr, HirStmt};
use crate::ast::{self, BinaryExpr, Stmt, UnaryExpr};
use crate::hir::UnaryOp;
use crate::syntax::SyntaxKind;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Database {
    pub exprs: Arena<HirExpr>,
}

impl Database {
    pub fn lower_stmt(&mut self, ast: ast::Stmt) -> Option<HirStmt> {
        let result = match ast {
            Stmt::VariableDef(var_def) => HirStmt::VariableDef {
                name: var_def.name()?.text().into(),
                value: self.lower_expr(var_def.value()),
            },
            Stmt::Expr(expr) => HirStmt::Expr(self.lower_expr(Some(expr))),
        };

        Some(result)
    }

    pub fn lower_expr(&mut self, ast: Option<ast::Expr>) -> HirExpr {
        ast.map_or_else(
            || HirExpr::Missing,
            |ast| match ast {
                ast::Expr::BinaryExpr(binary_expr) => self.lower_binary(binary_expr),
                ast::Expr::Literal(literal) => HirExpr::Literal { n: literal.parse() },
                ast::Expr::ParenExpr(paren_expr) => self.lower_expr(paren_expr.expr()),
                ast::Expr::UnaryExpr(unary_expr) => self.lower_unary(unary_expr),
                ast::Expr::VariableRef(variable_ref) => Self::lower_var_ref(variable_ref),
            },
        )
    }

    pub fn lower_binary(&mut self, ast: BinaryExpr) -> HirExpr {
        let op = match ast.op().unwrap().kind() {
            SyntaxKind::Plus => BinaryOp::Add,
            SyntaxKind::Minus => BinaryOp::Sub,
            SyntaxKind::Star => BinaryOp::Mul,
            SyntaxKind::Slash => BinaryOp::Div,
            _ => unreachable!(),
        };

        let lhs = self.lower_expr(ast.lhs());

        let rhs = self.lower_expr(ast.rhs());
        HirExpr::Binary {
            op,
            lhs: self.exprs.alloc(lhs),
            rhs: self.exprs.alloc(rhs),
        }
    }

    pub fn lower_unary(&mut self, ast: UnaryExpr) -> HirExpr {
        let op = match ast.op().unwrap().kind() {
            SyntaxKind::Minus => UnaryOp::Neg,
            _ => unreachable!(),
        };

        let expr = self.lower_expr(ast.expr());

        HirExpr::Unary {
            op,
            expr: self.exprs.alloc(expr),
        }
    }

    fn lower_var_ref(ast: ast::VariableRef) -> HirExpr {
        HirExpr::VariableRef {
            var: ast.name().unwrap().text().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    fn parse(input: &str) -> ast::Root {
        ast::Root::cast(parser::parse(input).syntax()).unwrap()
    }

    fn check_stmt(input: &str, expected_hir: HirStmt) {
        let root = parse(input);
        let ast = root.stmts().next().unwrap();
        let hir = Database::default().lower_stmt(ast).unwrap();

        assert_eq!(hir, expected_hir);
    }

    fn check_expr(input: &str, expected_hir: HirExpr, expected_database: Database) {
        let root = parse(input);
        let first_stmt = root.stmts().next().unwrap();

        let ast::Stmt::Expr(ast) = first_stmt else {
            unreachable!()
        };

        let mut database = Database::default();

        let hir = database.lower_expr(Some(ast));

        assert_eq!(hir, expected_hir);
        assert_eq!(database, expected_database);
    }

    #[test]
    fn lower_variable_def() {
        check_stmt(
            "let foo = bar",
            HirStmt::VariableDef {
                name: "foo".into(),
                value: HirExpr::VariableRef { var: "bar".into() },
            },
        );
    }

    #[test]
    fn lower_expr_stmt() {
        check_stmt("123", HirStmt::Expr(HirExpr::Literal { n: Some(123) }));
    }

    #[test]
    fn lower_binary_expr() {
        let mut exprs = Arena::new();

        let lhs = exprs.alloc(HirExpr::Literal { n: Some(1) });
        let rhs = exprs.alloc(HirExpr::Literal { n: Some(2) });

        check_expr(
            "1+2",
            HirExpr::Binary {
                op: BinaryOp::Add,
                lhs,
                rhs,
            },
            Database { exprs },
        );
    }
    #[test]
    fn lower_literal() {
        check_expr(
            "999",
            HirExpr::Literal { n: Some(999) },
            Database::default(),
        );
    }

    #[test]
    fn lower_paren_expr() {
        check_expr(
            "((((abc))))",
            HirExpr::VariableRef { var: "abc".into() },
            Database::default(),
        );
    }

    #[test]
    fn lower_unary_expr() {
        let mut exprs = Arena::new();
        let ten = exprs.alloc(HirExpr::Literal { n: Some(10) });

        check_expr(
            "-10",
            HirExpr::Unary {
                op: UnaryOp::Neg,
                expr: ten,
            },
            Database { exprs },
        );
    }

    #[test]
    fn lower_variable_ref() {
        check_expr(
            "foo",
            HirExpr::VariableRef { var: "foo".into() },
            Database::default(),
        );
    }

    #[test]
    fn lower_variable_def_without_name() {
        let root = parse("let = 10");
        let ast = root.stmts().next().unwrap();

        assert!(Database::default().lower_stmt(ast).is_none());
    }

    #[test]
    fn lower_variable_def_without_value() {
        check_stmt(
            "let a =",
            HirStmt::VariableDef {
                name: "a".into(),
                value: HirExpr::Missing,
            },
        );
    }

    #[test]
    fn lower_binary_expr_without_rhs() {
        let mut exprs = Arena::new();
        let lhs = exprs.alloc(HirExpr::Literal { n: Some(10) });
        let rhs = exprs.alloc(HirExpr::Missing);
        check_expr(
            "10 -",
            HirExpr::Binary {
                op: BinaryOp::Sub,
                lhs,
                rhs,
            },
            Database { exprs },
        );
    }

    #[test]
    fn lower_unary_without_expr() {
        let mut exprs = Arena::new();
        let expr = exprs.alloc(HirExpr::Missing);

        check_expr(
            "-",
            HirExpr::Unary {
                op: UnaryOp::Neg,
                expr,
            },
            Database { exprs },
        );
    }
}
