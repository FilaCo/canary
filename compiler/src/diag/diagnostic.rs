#[salsa::accumulator]
#[derive(Clone)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub msg: String,
}

#[derive(Clone, Debug)]
pub enum DiagnosticKind {
    Error,
    Warning,
    Notice,
}
