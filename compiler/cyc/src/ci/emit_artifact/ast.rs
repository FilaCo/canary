use cyc_ir::syntax::Score;

use crate::ci::{Canary, EmitArtifact, EmitKind};

impl EmitArtifact for Score {
    const KIND: EmitKind = EmitKind::Ast;

    fn render(&self, ci: &Canary) -> String {
        todo!()
    }
}
