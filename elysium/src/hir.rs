use crate::{ast::{self, Expr, Literal}, syntax, Stmt};
use smartstring::alias::String;

#[derive(Debug)]
pub enum HirStmt {
    VariableDef { name: String, value: HirExpr },
    Expr(HirExpr),
}

impl HirStmt {
    fn lower(ast: Stmt) -> Option<Self> {
        match ast {
            Stmt::VariableDef(ast) => Some(Self::VariableDef {
                name: ast.name()?.text().into(),
                value: HirExpr::lower(ast.value()),
            }),

                  Stmt::Expr(expr) => Some(Self::Expr(HirExpr::lower(Some(expr)))),
        }
    }
}

#[derive(Debug)]
pub enum HirExpr {
    Binary {
        op: BinaryOp,
        lhs: Box<Self>,
        rhs: Box<Self>,
    },
    Literal {
        n: u64,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Self>,
    },
    VariableRef {
        var: String,
    },
    Missing,
}

impl HirExpr {
    fn lower(ast: Option<Expr>) -> Self {
        ast.map_or_else(|| Self::Missing, |ast| match ast {
                Expr::BinaryExpr(binary_expr) => Self::lower_binary(binary_expr),
                Expr::Literal(literal) => Self::Literal { n: literal.parse() },
                Expr::ParenExpr(paren_expr) => Self::lower(paren_expr.expr()),
                Expr::UnaryExpr(unary_expr) => Self::lower_unary(unary_expr),
                Expr::VariableRef(variable_ref) => Self::VariableRef { var: variable_ref.name().into() },
            })
    }

    fn lower_binary(ast: ast::BinaryExpr) -> Self {
        let op = match ast.op().unwrap().kind() {
            syntax::SyntaxKind::Plus => BinaryOp::Add,
            
            syntax::SyntaxKind::Minus => BinaryOp::Sub,

            syntax::SyntaxKind::Star => BinaryOp::Mul,

            syntax::SyntaxKind::Slash => BinaryOp::Div,
            _ => unreachable!()
    };

        Self::Binary {
            op,
            lhs: Box::new(Self::lower(ast.lhs())),
            rhs: Box::new(Self::lower(ast.rhs()))
        }
    }

    fn lower_unary(ast: ast::UnaryExpr) -> Self {
        let op = match ast.op().unwrap().kind() {
            syntax::SyntaxKind::Minus => UnaryOp::Neg,
            _ => unreachable!()
       };

       Self::Unary { op, expr: Box::new(Self::lower(ast.expr())) }
    }
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum UnaryOp {
    Neg,
}


pub fn lower(ast: ast::Root) -> impl Iterator<Item = HirStmt> {
    ast.stmts().filter_map(HirStmt::lower)
}
