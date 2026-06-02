use std::path::Path;

use salsa::Storage;

use crate::{ci::CanaryConfig, db::CanaryDb};

#[salsa::db]
pub(super) struct CanaryDbImpl {
    storage: Storage<Self>,
    cfg: CanaryConfig,
}

impl CanaryDbImpl {
    pub fn new(cfg: CanaryConfig) -> Self {
        Self {
            storage: Storage::default(),
            cfg,
        }
    }
}

#[salsa::db]
impl CanaryDb for CanaryDbImpl {
    fn input(&self) -> &Path {
        self.cfg.input.as_path()
    }
}

impl salsa::Database for CanaryDbImpl {}
