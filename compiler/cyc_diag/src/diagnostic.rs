use DiagnosticLevel::*;
use cyc_ir::source::Span;

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub lvl: DiagnosticLevel,
    pub msg: String,
    pub primary_span: Span,
    pub labels: Vec<Label>,
}

impl Diagnostic {
    pub fn new(lvl: DiagnosticLevel, primary_span: Span, msg: impl Into<String>) -> Self {
        Self {
            lvl,
            primary_span,
            msg: msg.into(),
            labels: Vec::new(),
        }
    }

    pub fn error(primary_span: Span, msg: impl Into<String>) -> Self {
        Self::new(Error, primary_span, msg)
    }

    pub fn warning(primary_span: Span, msg: impl Into<String>) -> Self {
        Self::new(Warning, primary_span, msg)
    }

    pub fn notice(primary_span: Span, msg: impl Into<String>) -> Self {
        Self::new(Notice, primary_span, msg)
    }

    pub fn label(mut self, span: Span, msg: impl Into<String>) -> Self {
        self.labels.push(Label {
            span,
            msg: msg.into(),
        });
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Notice,
}

#[derive(Debug, Clone)]
pub struct Label {
    pub msg: String,
    pub span: Span,
}
