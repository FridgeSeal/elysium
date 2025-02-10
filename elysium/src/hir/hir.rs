use super::Database;
use crate::ast;
use la_arena::Idx;
use smartstring::alias::String;

type ExprIdx = Idx<HirExpr>;

#[derive(Debug, PartialEq, Eq)]
pub enum HirStmt {
    VariableDef { name: String, value: HirExpr },
    Expr(HirExpr),
}

#[derive(Debug, PartialEq, Eq)]
pub enum HirExpr {
    Binary {
        op: BinaryOp,
        lhs: ExprIdx,
        rhs: ExprIdx,
    },
    Literal {
        n: Option<u64>,
    },
    Unary {
        op: UnaryOp,
        expr: ExprIdx,
    },
    VariableRef {
        var: String,
    },
    Missing,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
}

pub fn lower(ast: ast::Root) -> (Database, Vec<HirStmt>) {
    let mut db = Database::default();
    let stmts = ast.stmts().filter_map(|stmt| db.lower_stmt(stmt)).collect();
    (db, stmts)
}
