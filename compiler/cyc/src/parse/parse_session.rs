use cyc_diag::DiagnosticContext;
use cyc_ir::syntax::SymbolInterner;

#[derive(Clone, Copy, Debug)]
pub struct ParseSession<'ci> {
    pub sym_intern: &'ci SymbolInterner,
    pub dcx: &'ci DiagnosticContext,
}
