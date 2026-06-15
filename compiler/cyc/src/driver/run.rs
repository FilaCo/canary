use std::{
    path::PathBuf,
    process::{ExitCode, Termination},
};

use crate::{
    ci::{CanaryConfig, EarlyDiagnosticContext, FatalErrorMarker, catch_fatal_errors, run_ci},
    driver::CanaryDriver,
    passes,
};

impl CanaryDriver {
    pub fn run(self) -> ExitCode {
        install_ice_hook();
        catch_with_exit_code(|| self.run_impl())
    }

    fn run_impl(self) -> ExitCode {
        let early_diag_ctx = EarlyDiagnosticContext::new();

        let input = self.input.canonicalize().unwrap_or_else(|e| {
            early_diag_ctx.fatal(std::format!(
                "unable to canonicalize path `{}`: {e}",
                self.input.to_string_lossy()
            ))
        });

        // TODO: replace hardcoded "default" with some meaningful value if possible
        let score_name = self.nest_name.unwrap_or(String::from("default"));

        let dump_kinds = self
            .dump
            .iter()
            .map(|arg| {
                (
                    arg.kind,
                    arg.file.clone().unwrap_or_else(|| {
                        PathBuf::from(format!("{}.{}", score_name, arg.kind.default_file_ext()))
                    }),
                )
            })
            .collect();

        let cfg = CanaryConfig {
            input,
            nest_name: score_name,
            dump: dump_kinds,
        };

        match run_ci(cfg, |ci| {
            let _ = passes::parse_nest(ci);
        }) {
            Ok(_) => ExitCode::SUCCESS,
            Err(_errors_reported) => ExitCode::FAILURE,
        }
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
