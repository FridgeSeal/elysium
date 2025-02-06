use crate::{ast::Expr, Stmt};
use smartstring::alias::String;

#[derive(Debug)]
pub enum HirStmt {
    VariableDef { name: String, value: Expr },
    Expr(Expr),
}

impl HirStmt {
    fn lower(ast: Stmt) -> Option<Self> {
        match ast {
            Stmt::VariableDef(ast) => Some(Self::VariableDef {
                name: ast.name()?.text().into(),
                value: Expr::lower(ast.value()),
            }),
            Stmt::Expr(expr) => Some(Self::Expr(Expr::lower(Some(expr)))),
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

impl Expr {
    fn lower(ast: Option<Expr>) -> Self {
        if let Some(ast) = ast {
            match ast {
                Expr::BinaryExpr(binary_expr) => todo!(),
                Expr::Literal(literal) => todo!(),
                Expr::ParenExpr(paren_expr) => todo!(),
                Expr::UnaryExpr(unary_expr) => todo!(),
                Expr::VariableRef(variable_ref) => todo!(),
            }
        } else {
            Self::Missing
        }
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
