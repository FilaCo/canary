use std::sync::Arc;

use cyc_ir::{source::SourceFile, syntax::Tokens};

use crate::{ci::Canary, parse::Lexer};

pub fn lex_file(ci: &Canary, src: Arc<SourceFile>) -> Tokens {
    Lexer::tokenize(&src.contents, src.span.lo, ci.psess())
}
