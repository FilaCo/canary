use crate::ir::source::Span;

#[salsa::tracked(debug)]
pub struct CanaryFile<'db> {
    #[tracked]
    #[returns(ref)]
    pub statements: Vec<Statement<'db>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, salsa::Update)]
pub struct Statement<'db> {
    pub kind: StatementKind<'db>,
    pub span: Span<'db>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, salsa::Update)]
pub enum StatementKind<'db> {
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
