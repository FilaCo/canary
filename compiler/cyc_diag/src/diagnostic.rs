use DiagnosticKind::*;
use cyc_ir::source::Span;

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub msg: String,
    pub span: Span,
    pub labels: Vec<Label>,
    pub notes: Vec<String>,
}

impl Diagnostic {
    pub fn new(kind: DiagnosticKind, span: Span, msg: impl Into<String>) -> Self {
        Self {
            kind,
            span,
            msg: msg.into(),
            labels: Vec::new(),
            notes: Vec::new(),
        }
    }

    pub fn error(span: Span, msg: impl Into<String>) -> Self {
        Self::new(Error, span, msg)
    }

    pub fn warning(span: Span, msg: impl Into<String>) -> Self {
        Self::new(Warning, span, msg)
    }

    pub fn notice(span: Span, msg: impl Into<String>) -> Self {
        Self::new(Notice, span, msg)
    }

    pub fn label(mut self, span: Span, msg: impl Into<String>) -> Self {
        self.labels.push(Label {
            span,
            msg: msg.into(),
        });
        self
    }

    pub fn note(mut self, msg: impl Into<String>) -> Self {
        self.notes.push(msg.into());
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticKind {
    Error,
    Warning,
    Notice,
}

#[derive(Debug, Clone)]
pub struct Label {
    pub msg: String,
    pub span: Span,
}
