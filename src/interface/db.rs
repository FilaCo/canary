use std::fmt::Arguments;

use crate::EarlyDcx;

#[salsa::db]
pub trait CanaryDb: salsa::Database {
    fn report_fatal(&self, msg: Arguments<'_>) -> !;
}

#[salsa::db]
pub struct CanaryDbImpl {
    storage: salsa::Storage<Self>,
    early_dcx: EarlyDcx,
}

impl CanaryDbImpl {
    pub fn new() -> Self {
        Self {
            storage: salsa::Storage::default(),
            early_dcx: EarlyDcx,
        }
    }
}

impl Default for CanaryDbImpl {
    fn default() -> Self {
        Self::new()
    }
}

#[salsa::db]
impl salsa::Database for CanaryDbImpl {}

#[salsa::db]
impl CanaryDb for CanaryDbImpl {
    fn report_fatal(&self, msg: Arguments<'_>) -> ! {
        self.early_dcx.report_fatal(msg)
    }
}
