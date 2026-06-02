use crate::{
    ci::{CanaryConfig, CanaryDbImpl},
    db::CanaryDb,
};

pub fn run_ci<R>(cfg: CanaryConfig, f: impl Fn(&dyn CanaryDb) -> R) -> R {
    f(&CanaryDbImpl::new(cfg))
}
