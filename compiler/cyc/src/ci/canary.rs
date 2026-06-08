use std::path::PathBuf;

use crate::ci::{EarlyDiagnosticContext, SourceMap};
use cyc_diag::DiagnosticContext;

pub fn run_ci<R: Send>(cfg: CanaryConfig, f: impl FnOnce(&Canary) -> R + Send) -> R {
    let ci = Canary {
        cfg,
        source_map: SourceMap::new(),
        diag_ctx: DiagnosticContext::new(),
    };

    f(&ci)
}

#[derive(Debug)]
pub struct Canary {
    pub cfg: CanaryConfig,
    pub source_map: SourceMap,
    pub diag_ctx: DiagnosticContext,
}

#[derive(Debug)]
pub struct CanaryConfig {
    pub input: PathBuf,
    pub early_diag_ctx: EarlyDiagnosticContext,
}
