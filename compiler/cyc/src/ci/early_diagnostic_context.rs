use yansi::Paint;

#[derive(Debug)]
pub struct EarlyDiagnosticContext;

impl EarlyDiagnosticContext {
    pub fn new() -> Self {
        Self
    }

    pub fn error(&self, msg: impl std::fmt::Display) -> ! {
        eprintln!("{}: {msg}", "error".bright_red().bold());
        std::process::exit(1);
    }

    pub fn warning(&self, msg: impl std::fmt::Display) {
        eprintln!("{}: {msg}", "warning".bright_yellow().bold());
    }
}

impl Default for EarlyDiagnosticContext {
    fn default() -> Self {
        Self::new()
    }
}
