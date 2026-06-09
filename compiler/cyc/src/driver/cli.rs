use std::path::PathBuf;

use clap::{Parser, ValueEnum};

use crate::ci::EmitKind;

#[derive(Parser, Debug)]
#[command(version)]
pub struct CanaryDriver {
    // Input source.
    pub input: PathBuf,

    /// Specify the name of the score.
    #[arg(long, value_name = "NAME")]
    pub score_name: Option<String>,

    /// Kind of output for the compiler to emit
    /// Each KIND has the default FILE name:
    /// * tokens - SCORE_NAME.tok
    /// * ast    - SCORE_NAME.ast
    #[arg(long, value_name = "KIND[=FILE]", value_parser = parse_emit, verbatim_doc_comment)]
    pub emit: Vec<EmitArg>,
}

impl Default for CanaryDriver {
    fn default() -> Self {
        CanaryDriver::parse()
    }
}

#[derive(Debug, Clone, Eq)]
pub struct EmitArg {
    pub kind: EmitKind,
    pub file: Option<PathBuf>,
}

impl PartialEq for EmitArg {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

/// Parse a string argument for the `--emit` option.
///
/// The string must follow the format `KIND[=FILE]` where:
/// - `KIND` is one of the [`EmitKind`] variants (case-insensitive)
/// - `FILE` is an optional output file path
///
/// # Examples
/// - `"tokens"` -> Emit tokens with default filename
/// - `"ast=output.ast"` -> Emit AST to `output.ast`
/// - `"Tokens=debug.tok"` -> Case-insensitive, emit tokens to `debug.tok`
///
/// # Arguments
///
/// * `s` - The string to parse in `KIND[=FILE]` format
///
/// # Returns
///
/// Returns `Ok(EmitArg)` if parsing succeeds, or an error string describing
/// what went wrong.
///
/// # Errors
///
/// Returns an error string if:
/// - `KIND` is missing or empty (e.g., `""`, `"=file"`)
/// - `KIND` is not a valid [`EmitKind`] variant
/// - `FILE` is specified but empty (e.g., `"tokens="`, `"ast=   "`)
/// - There are multiple `=` characters (e.g., `"tokens=a=b"`)
fn parse_emit(s: &str) -> Result<EmitArg, String> {
    // TODO: more verbose error messages, e.g. for `invalid=` input provide 2 error
    // notes We use `split('=')` instead of `split_once('=')` to detect
    // multiple `=` characters and provide better error messages.
    let mut parts = s.split('=');
    let kind = parts
        .next()
        .ok_or(String::from("KIND is empty"))
        .and_then(|s| {
            if s.is_empty() {
                Err(String::from("KIND is empty"))
            } else {
                EmitKind::from_str(s, true)
            }
        });
    let file = parts
        .next()
        .map(|s| s.trim())
        .map(|s| {
            if s.is_empty() {
                Err(String::from("FILE is empty"))
            } else {
                Ok(PathBuf::from(s))
            }
        })
        .transpose();

    if parts.count() > 0 {
        return Err(String::from(
            "too many '=' characters, expected format KIND[=FILE]",
        ));
    }

    Ok(EmitArg {
        kind: kind?,
        file: file?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_emit() {
        let cases = [
            ("tokens", EmitKind::Tokens),
            ("Tokens", EmitKind::Tokens),
            ("TOKENS", EmitKind::Tokens),
            ("ast", EmitKind::Ast),
            ("Ast", EmitKind::Ast),
            ("AST", EmitKind::Ast),
        ];

        for (input, expected_kind) in cases {
            let result = parse_emit(input);
            let emit_arg = result.expect("parse_emit failed");
            assert_eq!(emit_arg.kind, expected_kind);
            assert_eq!(emit_arg.file, None);
        }
    }

    #[test]
    fn test_parse_emit_with_file() {
        let cases = [
            ("tokens=output.tok", EmitKind::Tokens, "output.tok"),
            ("ast=debug.ast", EmitKind::Ast, "debug.ast"),
            ("Tokens=MYTOKENS.TOK", EmitKind::Tokens, "MYTOKENS.TOK"),
        ];

        for (input, expected_kind, expected_file) in cases {
            let result = parse_emit(input);
            let emit_arg = result.expect("parse_emit failed");
            assert_eq!(emit_arg.kind, expected_kind);
            assert_eq!(emit_arg.file, Some(PathBuf::from(expected_file)),);
        }
    }

    #[test]
    fn test_parse_emit_errors() {
        // Invalid emission kind
        assert!(parse_emit("invalid").is_err_and(|e| e.contains("invalid variant: invalid")));
        assert!(parse_emit("invalid=").is_err_and(|e| e.contains("invalid variant: invalid")));
        assert!(parse_emit("=output.tok").is_err_and(|e| e.contains("KIND is empty")));
        assert!(parse_emit("=").is_err_and(|e| e.contains("KIND is empty")));

        // Too many '=' characters
        assert!(
            parse_emit("tokens=file.tok=extra")
                .is_err_and(|e| e.contains("too many '=' characters, expected format KIND[=FILE]"))
        );

        // Empty file path
        assert!(parse_emit("tokens=").is_err_and(|e| e.contains("FILE is empty")));
        assert!(parse_emit("ast=   ").is_err_and(|e| e.contains("FILE is empty")));
    }

    #[test]
    fn test_emit_arg_equality() {
        let arg1 = EmitArg {
            kind: EmitKind::Tokens,
            file: Some(PathBuf::from("file1.tok")),
        };

        let arg2 = EmitArg {
            kind: EmitKind::Tokens,
            file: Some(PathBuf::from("file2.tok")),
        };

        let arg3 = EmitArg {
            kind: EmitKind::Ast,
            file: Some(PathBuf::from("file.ast")),
        };

        // Same kind, different files should be equal
        assert_eq!(arg1, arg2);

        // Different kinds should not be equal
        assert_ne!(arg1, arg3);
        assert_ne!(arg2, arg3);

        // Reflexivity
        assert_eq!(arg1, arg1);
        assert_eq!(arg2, arg2);
        assert_eq!(arg3, arg3);
    }
}
