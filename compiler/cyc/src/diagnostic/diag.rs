use crate::{diagnostic::DiagCtx, ir::source::Span};

use DiagKind::*;

#[derive(Debug, Clone)]
pub struct Diag {
    pub kind: DiagKind,
    pub msg: String,
    pub span: Span,
    pub labels: Vec<Label>,
    pub notes: Vec<String>,
}

impl Diag {
    pub fn new(kind: DiagKind, span: Span, msg: impl Into<String>) -> Self {
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

    pub fn accumulate(self, ctx: &DiagCtx) {
        ctx.accumulate(self);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagKind {
    Error,
    Warning,
    Notice,
}

#[derive(Debug, Clone)]
pub struct Label {
    pub msg: String,
    pub span: Span,
}
