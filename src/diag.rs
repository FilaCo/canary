use std::fmt::Display;

#[salsa::accumulator]
#[derive(Debug)]
pub struct Diag {
    pub lvl: DiagLvl,
    pub msg: String,
    // pub primary_span: Span,
    pub labels: Vec<Label>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagLvl {
    Error,
    Warning,
    Note,
}

#[derive(Debug, Clone)]
pub struct Label {
    pub msg: String,
    // pub span: Span,
}

impl Label {
    pub fn new(
        msg: &dyn Display,
        // span: Span
    ) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}

use DiagLvl::*;

impl Diag {
    pub fn error(
        // primary_span: Span,
        msg: impl Display,
    ) -> Self {
        Self::new(
            Error, // primary_span,
            &msg,
        )
    }

    pub fn warning(
        // primary_span: Span,
        msg: impl Display,
    ) -> Self {
        Self::new(
            Warning, // primary_span,
            &msg,
        )
    }

    pub fn note(
        // primary_span: Span,
        msg: impl Display,
    ) -> Self {
        Self::new(
            Note, //primary_span,
            &msg,
        )
    }

    pub fn label(
        mut self,
        // span: Span,
        msg: impl Display,
    ) -> Self {
        let label = Label::new(
            // span,
            &msg,
        );
        self.labels.push(label);

        self
    }

    fn new(
        lvl: DiagLvl,
        // primary_span: Span,
        msg: &dyn Display,
    ) -> Self {
        Self {
            lvl,
            msg: msg.to_string(),
            labels: Vec::new(),
        }
    }
}
