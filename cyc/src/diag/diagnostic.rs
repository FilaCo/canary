use crate::ir::source::Span;

#[derive(Debug)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub msg: String,
    pub span: Span,
}

#[derive(Debug)]
pub enum DiagnosticKind {
    Error,
    Warning,
    Notice,
}
