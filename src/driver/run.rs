use std::{
    io::{Write, stdin, stdout},
    process::{ExitCode, Termination},
    time::{Duration, Instant},
};

use clap::Parser;

use crate::{
    FatalErrorMarker, catch_fatal_errors,
    driver::CanaryCli,
    interface::{CanaryDb, CanaryDbImpl},
};

pub fn run() -> ExitCode {
    install_ice_hook();

    catch_with_exit_code(|| {
        use yansi::Paint;

        let (exit_code, elapsed) = measure_duration(run_driver);
        println!(
            "{} in {:.3}s",
            "Finished".bright_green().bold(),
            elapsed.as_secs_f32()
        );
        exit_code
    })
}

fn run_driver() -> ExitCode {
    let cli = CanaryCli::parse();

    match &cli.cmd {
        Some(_) => todo!(),
        None => run_repl(cli),
    }
}

fn run_repl(_: CanaryCli) -> ExitCode {
    let db = CanaryDbImpl::default();

    let input = stdin();
    let mut line = String::new();
    let mut output = stdout();
    loop {
        write!(&mut output, "🐣 >>> ").unwrap_or_else(|e| {
            db.report_fatal(format_args!("unable to write prompt invitation: {e}"))
        });
        output.flush().unwrap_or_else(|e| {
            db.report_fatal(format_args!("unable to flush output writer: {e}"))
        });

        let bytes_read = input
            .read_line(&mut line)
            .unwrap_or_else(|e| db.report_fatal(format_args!("unable to read line: {e}")));

        if bytes_read == 0 {
            println!();
            break;
        }

        line.clear();
    }
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
