use std::path::Path;

use salsa::Database;

#[salsa::db]
pub trait CanaryDb: Database {
    fn input(&self) -> &Path;
}
