use cyc_ir::syntax::Nest;

use crate::ci::{Canary, EmitArtifact, EmitKind};

impl EmitArtifact for Nest {
    const KIND: EmitKind = EmitKind::Ast;

    fn render(&self, ci: &Canary) -> String {
        todo!()
    }
}
