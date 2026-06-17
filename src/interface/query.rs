use crate::{interface::CanaryDb, ir::source};

#[salsa::tracked]
pub fn parse_submission(db: &dyn CanaryDb, submission: source::Submission) {}
