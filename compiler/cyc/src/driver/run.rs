use std::path::PathBuf;

use crate::{
    ci::{CanaryConfig, EarlyDiagnosticContext, run_ci},
    driver::CanaryDriver,
};

impl CanaryDriver {
    pub fn run(self) {
        let early_diag_ctx = EarlyDiagnosticContext::new();
        let Self { input, .. } = self;

        let cfg = CanaryConfig {
            input,
            early_diag_ctx,
        };
        run_ci(cfg, |ci| {
            let input_src_file = match ci.source_map.add(&ci.cfg.input) {
                Ok(src) => src,
                Err(e) => ci.cfg.early_diag_ctx.error(e),
            };
        });
    }
}
