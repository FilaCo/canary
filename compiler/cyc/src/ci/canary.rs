use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::{
    ci::{
        EarlyDiagnosticContext, ErrorsReported, SourceMap,
        diagnostic_emitter::HumanReadableDiagnosticEmitter,
    },
    parse::ParseSession,
};
use clap::ValueEnum;
use cyc_diag::{DiagnosticContext, DiagnosticEmitter};
use cyc_ir::syntax::SymbolInterner;

pub fn run_ci<R: Send>(
    cfg: CanaryConfig,
    f: impl FnOnce(&Canary) -> R + Send,
) -> Result<R, ErrorsReported> {
    let early_diag_ctx = EarlyDiagnosticContext::new();
    let dcx = DiagnosticContext::new();
    let sym_interner = SymbolInterner::new();
    let psess = ParseSession::new(&sym_interner, &dcx);
    let ci = Canary {
        cfg,
        sm: SourceMap::new(),
        early_dcx: early_diag_ctx,
        dcx,
        sym_interner,
    };
    let _emit_on_drop_guard = EmitOnDrop(&ci);

    let res = f(&ci);

    if ci.dcx.has_errors() {
        Err(ErrorsReported)
    } else {
        Ok(res)
    }
}

#[derive(Debug)]
pub struct Canary {
    pub cfg: CanaryConfig,
    pub sm: SourceMap,
    pub early_dcx: EarlyDiagnosticContext,
    pub dcx: DiagnosticContext,
    pub sym_interner: SymbolInterner,
}

impl Canary {
    pub fn psess<'ci>(&'ci self) -> ParseSession<'ci> {
        ParseSession::new(&self.sym_interner, &self.dcx)
    }
}

#[derive(Debug)]
pub struct CanaryConfig {
    pub input: PathBuf,
    pub emit_targets: HashMap<EmitKind, PathBuf>,
    pub seed_name: String,
}

/// Kinds of output `cyc` can emit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub enum EmitKind {
    /// Emit the token stream after lexical analysis.
    Tokens,
    /// Emit the abstract syntax tree after parsing.
    Ast,
}

impl EmitKind {
    pub fn default_file_ext(&self) -> &'static str {
        match self {
            EmitKind::Tokens => "tok",
            EmitKind::Ast => "ast",
        }
    }
}

#[derive(Debug)]
struct EmitOnDrop<'ci>(&'ci Canary);

impl Drop for EmitOnDrop<'_> {
    fn drop(&mut self) {
        HumanReadableDiagnosticEmitter::new(&self.0.sm).emit_all(&self.0.dcx.accumulated());
    }
}
