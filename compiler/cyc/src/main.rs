use std::process::ExitCode;

use cyc::driver::CanaryDriver;

fn main() -> ExitCode {
    CanaryDriver::default().run()
}
