use crate::interface::CanaryDb;

#[salsa::tracked]
pub fn parse_line(db: &dyn CanaryDb) {}
