use std::{
    fmt::{Display, Formatter},
    panic::{AssertUnwindSafe, catch_unwind, resume_unwind},
};

/// Used as a return value to signify a fatal error occurred.
#[derive(Copy, Clone, Debug)]
#[must_use]
pub struct FatalError;

pub(crate) struct FatalErrorMarker;

impl FatalError {
    pub fn raise(self) -> ! {
        resume_unwind(Box::new(FatalErrorMarker))
    }
}

impl Display for FatalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "fatal error")
    }
}

impl std::error::Error for FatalError {}

/// Runs a closure and catches unwinds triggered by fatal errors.
///
/// The compiler currently unwinds with a special sentinel value to abort
/// compilation on fatal errors. This function catches that sentinel and turns
/// the panic into a `Result` instead.
pub fn catch_fatal_errors<F: FnOnce() -> R, R>(f: F) -> Result<R, FatalError> {
    catch_unwind(AssertUnwindSafe(f)).map_err(|value| {
        if value.is::<FatalErrorMarker>() {
            FatalError
        } else {
            resume_unwind(value);
        }
    })
}
