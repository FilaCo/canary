use crate::ci::{Canary, CanaryConfig};

pub fn run_ci<R>(cfg: CanaryConfig, f: impl Fn(&Canary) -> R) -> R {
    f(&Canary::new(cfg))
}
