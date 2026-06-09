use std::sync::Arc;

use cyc_ir::{source::SourceFile, syntax::CanaryFile};

use crate::{
    ci::{Canary, emit_artifact},
    parse::Parser,
    passes::lex_file,
};

pub fn parse_file(ci: &Canary, src: Arc<SourceFile>) -> CanaryFile {
    let tokens = lex_file(ci, src);
    emit_artifact(ci, &tokens);

    let mut parser = Parser::new(&tokens, ci.psess());
    parser.parse_file()
}
