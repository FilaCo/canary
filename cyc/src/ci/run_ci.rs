use crate::ci::{Canary, CanaryConfig};

pub fn run_ci<R: Send>(cfg: CanaryConfig, f: impl FnOnce(&Canary) -> R + Send) -> R {
    todo!()
}
