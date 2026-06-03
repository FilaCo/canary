use crate::ir::source::Span;

#[derive(Debug)]
pub struct CanaryFile {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum StmtKind {
    Expr(Expr),
}

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum ExprKind {
    Binary {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Grouped {
        lparen_sp: Span,
        expr: Box<Expr>,
        rparen_sp: Span,
    },
    LitConst,
}
