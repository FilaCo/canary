use std::path::PathBuf;

use crate::ci::{
    EarlyDiagnosticContext, ErrorsReported, SourceMap,
    diagnostic_emitter::HumanReadableDiagnosticEmitter,
};
use cyc_diag::{DiagnosticContext, DiagnosticEmitter};

pub fn run_ci<R: Send>(
    cfg: CanaryConfig,
    f: impl FnOnce(&Canary) -> R + Send,
) -> Result<R, ErrorsReported> {
    let early_diag_ctx = EarlyDiagnosticContext::new();
    let ci = Canary {
        cfg,
        source_map: SourceMap::new(),
        early_diag_ctx,
        diag_ctx: DiagnosticContext::new(),
    };
    let _emit_on_drop_guard = EmitOnDrop(&ci);

    let res = f(&ci);

    if ci.diag_ctx.has_errors() {
        Err(ErrorsReported)
    } else {
        Ok(res)
    }
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

struct EmitOnDrop<'a>(&'a Canary);

impl Drop for EmitOnDrop<'_> {
    fn drop(&mut self) {
        HumanReadableDiagnosticEmitter::new(&self.0.source_map)
            .emit_all(&self.0.diag_ctx.accumulated());
    }
}
