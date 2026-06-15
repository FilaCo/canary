use cyc_ir::syntax::Nest;

use crate::ci::{Canary, DumpKind, EmitArtifact};

impl EmitArtifact for Nest {
    const KIND: DumpKind = DumpKind::Ast;

    fn render(&self, ci: &Canary) -> String {
        todo!()
    }
}
