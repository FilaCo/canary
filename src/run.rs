use std::{
    process::{ExitCode, Termination},
    time::{Duration, Instant},
};

use clap::Parser;
use yansi::Paint;

use crate::{FatalErrorMarker, catch_fatal_errors, cli::CanaryCli};

pub fn run() -> ExitCode {
    install_ice_hook();

    catch_with_exit_code(|| {
        let (exit_code, elapsed) = measure_duration(run_driver);
        println!(
            "{} in {:.2}s",
            "Finished".bold().bright_green(),
            elapsed.as_secs_f64()
        );
        exit_code
    })
}

fn run_driver() -> ExitCode {
    let cli = CanaryCli::parse();
    ExitCode::SUCCESS
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

fn measure_duration<T>(f: impl FnOnce() -> T) -> (T, Duration) {
    let now = Instant::now();
    let res = f();
    (res, now.elapsed())
}

fn catch_with_exit_code<T: Termination>(f: impl FnOnce() -> T) -> ExitCode {
    match catch_fatal_errors(f) {
        Ok(status) => status.report(),
        _ => ExitCode::FAILURE,
    }
}
