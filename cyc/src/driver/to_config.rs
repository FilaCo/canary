use crate::{ci::CanaryConfig, driver::CanaryDriver};

impl From<CanaryDriver> for CanaryConfig {
    fn from(value: CanaryDriver) -> Self {
        Self { input: value.input }
    }
}
