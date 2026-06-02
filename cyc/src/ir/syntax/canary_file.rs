use crate::ir::source::Span;

#[salsa::tracked(debug)]
pub struct CanaryFile<'db> {
    #[tracked]
    #[returns(ref)]
    pub stmts: Vec<Stmt<'db>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, salsa::Update)]
pub struct Stmt<'db> {
    pub kind: StmtKind<'db>,
    pub span: Span<'db>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, salsa::Update)]
pub enum StmtKind<'db> {
    Expr(Expr<'db>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, salsa::Update)]
pub struct Expr<'db> {
    pub kind: ExprKind<'db>,
    pub span: Span<'db>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, salsa::Update)]
pub enum ExprKind<'db> {
    Binary {
        lhs: Box<Expr<'db>>,
        rhs: Box<Expr<'db>>,
    },
    Grouped {
        expr: Box<Expr<'db>>,
    },
    LitConst,
}

#[salsa::interned(debug)]
pub struct Value<'db> {
    #[returns(ref)]
    pub value: String,
}
