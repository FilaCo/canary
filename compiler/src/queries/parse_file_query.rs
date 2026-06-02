use crate::{
    db::CanaryDb,
    ir::{source::SourceFile, syntax::CanaryFile},
};

#[salsa::tracked]
pub fn parse_file<'db>(db: &'db dyn CanaryDb, file: SourceFile) -> CanaryFile<'db> {
    CanaryFile::new(db, Vec::new())
}
