use std::path::PathBuf;

use crate::ci::{EarlyDiagnosticContext, SourceMap};
use cyc_diag::DiagnosticContext;

pub fn run_ci<R: Send>(cfg: CanaryConfig, f: impl FnOnce(&Canary) -> R + Send) -> R {
    let early_diag_ctx = EarlyDiagnosticContext::new();
    let ci = Canary {
        cfg,
        source_map: SourceMap::new(),
        early_diag_ctx,
        diag_ctx: DiagnosticContext::new(),
    };

    f(&ci)
}

#[derive(Debug)]
pub struct Canary {
    pub cfg: CanaryConfig,
    pub source_map: SourceMap,
    pub early_diag_ctx: EarlyDiagnosticContext,
    pub diag_ctx: DiagnosticContext,
}

#[derive(Debug)]
pub struct CanaryConfig {
    pub input: PathBuf,
}
