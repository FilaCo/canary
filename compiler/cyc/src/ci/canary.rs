use std::path::PathBuf;

use crate::{ci::SourceMap, diagnostic::DiagCtx};

pub fn run_ci<R: Send>(cfg: CanaryConfig, f: impl FnOnce(&Canary) -> R + Send) -> R {
    todo!()
}

#[derive(Debug)]
pub struct Canary {
    pub cfg: CanaryConfig,
    pub source_map: SourceMap,
    pub diag_ctx: DiagCtx,
}

#[derive(Debug)]
pub struct CanaryConfig {
    pub input: PathBuf,
}
