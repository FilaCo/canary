use std::fmt::Arguments;

use yansi::Paint;

use crate::FatalError;

#[derive(Debug)]
pub(super) struct EarlyDcx;

impl EarlyDcx {
    pub fn report_fatal(&self, msg: Arguments<'_>) -> ! {
        eprintln!("{}: {msg}", "error".bright_red().bold());
        FatalError.raise()
    }
}
