use crate::{
    db::CanaryDb,
    ir::{source::SourceFile, syntax::CanaryFile},
    parse::Parser,
};

#[salsa::tracked]
pub fn parse_file<'db>(db: &'db dyn CanaryDb, file: SourceFile) -> CanaryFile<'db> {
    Parser::new(db, file).canary_file()
}
