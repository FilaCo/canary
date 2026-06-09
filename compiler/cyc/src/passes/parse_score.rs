use cyc_ir::syntax::Score;

use crate::{
    ci::{Canary, emit_artifact},
    passes::parse_file,
};

pub fn parse_score(ci: &Canary) -> Score {
    let input = &ci.cfg.input;
    let sm = &ci.sm;
    let early_dcx = &ci.early_dcx;
    let src = sm.add(input).unwrap_or_else(|e| early_dcx.fatal(e));

    let score = Score {
        file: parse_file(ci, src),
    };
    emit_artifact(ci, &score);
    score
}
