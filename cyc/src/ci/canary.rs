use crate::ci::CanaryConfig;

pub struct Canary {
    cfg: CanaryConfig,
}

impl Canary {
    pub fn new(cfg: CanaryConfig) -> Self {
        Self { cfg }
    }
}
