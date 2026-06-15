use std::fmt::Write;

use cyc_ir::syntax::{TokenKind, Tokens};

use crate::ci::{Canary, EmitArtifact, DumpKind};

impl EmitArtifact for Tokens {
    const KIND: DumpKind = DumpKind::Tokens;

    fn render(&self, ci: &Canary) -> String {
        let sm = &ci.sm;
        // Resolve every token up front so we can align the position and kind
        // columns to their widest entry.
        let rows: Vec<(String, String, String)> = self
            .iter()
            .map(|token| {
                let kind = kind_label(&token.kind);
                match sm.resolve_span(token.span) {
                    Some((file, range)) => {
                        let (line, col) = file.line_col(token.span.lo);
                        let text = &file.contents[range];
                        let text = if text.is_empty() {
                            String::new()
                        } else {
                            format!("\"{}\"", text.escape_default())
                        };
                        (format!("{line}:{col}"), kind, text)
                    }
                    // A token from a loaded file should always resolve; mark it
                    // instead of panicking inside a debug dump.
                    None => ("?:?".to_string(), kind, String::new()),
                }
            })
            .collect();

        let pos_width = rows.iter().map(|(pos, ..)| pos.len()).max().unwrap_or(0);
        let kind_width = rows
            .iter()
            .map(|(_, kind, _)| kind.len())
            .max()
            .unwrap_or(0);

        let mut out = String::new();
        for (pos, kind, text) in &rows {
            if text.is_empty() {
                writeln!(out, "{pos:<pos_width$}  {kind}").expect("writing to a String");
            } else {
                writeln!(out, "{pos:<pos_width$}  {kind:<kind_width$}  {text}")
                    .expect("writing to a String");
            }
        }
        out
    }
}

fn kind_label(kind: &TokenKind) -> String {
    match kind {
        TokenKind::LitConst(lit) => format!("{:?}", lit.kind),
        other => format!("{other:?}"),
    }
}
