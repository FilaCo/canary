use cyc_ir::syntax::Seed;

use crate::{
    ci::Canary,
    parse::{ParseSession, Parser},
};

pub fn parse_seed(ci: &Canary) -> Seed {
    let sm = &ci.sm;
    let early_dcx = &ci.early_dcx;
    let src = sm.add(&ci.cfg.input).unwrap_or_else(|e| early_dcx.fatal(e));
    let psess = ParseSession {
        sym_intern: &ci.sym_interner,
        dcx: &ci.dcx,
    };
    let mut parser = Parser::new(&src.contents, src.span.lo, psess);

    Seed {
        file: parser.parse_file(),
    }
}
