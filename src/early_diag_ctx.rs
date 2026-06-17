use yansi::Paint;

use crate::FatalError;

#[derive(Debug)]
pub struct EarlyDiagnosticContext;

impl EarlyDiagnosticContext {
    pub fn fatal(&self, msg: impl std::fmt::Display) -> ! {
        eprintln!("{}: {msg}", "error".bright_red().bold());
        FatalError.raise()
    }
}
