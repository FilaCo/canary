use crate::ci::{Canary, DumpKind};

mod ast;
mod tokens;

pub trait EmitArtifact {
    const KIND: DumpKind;
    fn render(&self, ci: &Canary) -> String;
}

pub fn emit_artifact<A: EmitArtifact>(ci: &Canary, artifact: &A) {
    let Some(path) = ci.cfg.emit_targets.get(&A::KIND) else {
        return;
    };
    let contents = artifact.render(ci);
    std::fs::write(path, contents).unwrap_or_else(|e| ci.early_dcx.fatal(e));
}
