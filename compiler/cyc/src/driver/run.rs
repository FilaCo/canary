use std::process::{ExitCode, Termination};

use crate::{
    ci::{CanaryConfig, FatalErrorMarker, catch_fatal_errors, run_ci},
    driver::CanaryDriver,
};

impl CanaryDriver {
    pub fn run(self) -> ExitCode {
        install_ice_hook();
        catch_with_exit_code(|| self.run_impl())
    }

    fn run_impl(self) {
        let cfg = CanaryConfig { input: self.input };

        run_ci(cfg, |ci| {
            let input_file_path = ci.cfg.input.canonicalize().unwrap_or_else(|e| {
                ci.early_diag_ctx.fatal(std::format!(
                    "unable to canonicalize path `{}`: {e}",
                    ci.cfg.input.to_string_lossy()
                ))
            });

            let input_src_file = ci
                .source_map
                .add(&input_file_path)
                .unwrap_or_else(|e| ci.early_diag_ctx.fatal(e));
        });
    }
}

fn catch_with_exit_code<T: Termination>(f: impl FnOnce() -> T) -> ExitCode {
    match catch_fatal_errors(f) {
        Ok(status) => status.report(),
        _ => ExitCode::FAILURE,
    }
}

fn install_ice_hook() {
    let next_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        if info.payload().is::<FatalErrorMarker>() {
            // deliberate abort, already diagnosed
            return;
        }

        next_hook(info);
    }));
}
