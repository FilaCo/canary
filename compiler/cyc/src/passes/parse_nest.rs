use cyc_ir::syntax::Nest;

use crate::{ci::Canary, passes::parse_file};

pub fn parse_nest(ci: &Canary) -> Nest {
    let input = &ci.cfg.input;
    let sm = &ci.sm;
    let early_dcx = &ci.early_dcx;
    let src = sm.add(input).unwrap_or_else(|e| early_dcx.fatal(e));

    todo!()
}
