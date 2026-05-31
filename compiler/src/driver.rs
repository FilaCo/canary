use std::{
    fmt::Display,
    fs,
    io::{self, Write},
    path::PathBuf,
    str::{Chars, FromStr},
};

use clap::Parser;
use thiserror::Error;

#[derive(Parser, Debug)]
#[command(version)]
pub struct CanaryDriver {
    // Input source.
    #[arg(default_value_t = CanaryInput::Repl)]
    input: CanaryInput,
}

impl CanaryDriver {
    pub fn run(self) {
        match self.input {
            CanaryInput::Repl => Self::run_repl(),
            CanaryInput::File(file_name) => Self::run_file(file_name),
        }
    }

    fn run_repl() {
        let input = io::stdin();
        let mut line = String::new();
        let mut output = io::stdout();

        loop {
            write!(&mut output, "🐣 >>> ").expect("unable to write prompt invitation");
            output.flush().expect("unable to flush output writer");

            match input.read_line(&mut line) {
                Err(e) => eprintln!("{e}"),
                Ok(0) => break,
                Ok(_) => Self::run_source(&line),
            }

            line.clear();
        }
    }

    fn run_file(file_name: PathBuf) {
        let contents = fs::read_to_string(file_name).expect("unable to read input file");
        Self::run_source(&contents)
    }

    fn run_source(src: &str) {
        let mut lexer = Lexer::new(src);
        for t in lexer {
            println!("{:?}", t);
        }
    }
}

impl Default for CanaryDriver {
    fn default() -> Self {
        CanaryDriver::parse()
    }
}

#[derive(Clone, Debug)]
enum CanaryInput {
    Repl,
    File(PathBuf),
}

impl FromStr for CanaryInput {
    type Err = CanaryInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const REPL_INPUT: &str = "";
        match s {
            REPL_INPUT => Ok(CanaryInput::Repl),
            s => {
                let fpath = PathBuf::from_str(s)
                    .unwrap()
                    .canonicalize()
                    .map_err(CanaryInputError::InvalidInput)?;

                if fpath.is_file() {
                    Ok(CanaryInput::File(fpath))
                } else {
                    Err(CanaryInputError::NotSupportedInput(fpath))
                }
            }
        }
    }
}

impl Display for CanaryInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CanaryInput::Repl => write!(f, ""),
            CanaryInput::File(file_name) => write!(f, "{}", file_name.to_string_lossy()),
        }
    }
}

#[derive(Error, Debug)]
enum CanaryInputError {
    #[error("{0}")]
    InvalidInput(#[from] io::Error),
    #[error("unsupported input `{0}`")]
    NotSupportedInput(PathBuf),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Span {
    /// inclusive
    start: usize,
    /// exclusive
    end: usize,
}

impl Span {
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub const fn dummy() -> Self {
        Self::new(0, 0)
    }
}

#[derive(Debug)]
struct Diagnostic {
    kind: DiagnosticKind,
    msg: String,
    span: Span,
}

#[derive(Debug)]
enum DiagnosticKind {
    Error,
    Warning,
    Notice,
}

#[derive(Clone, Copy, Debug)]
enum TokenKind<'a> {
    Minus,
    Plus,
    Slash,
    Star,
    LParen,
    RParen,
    Int { value: &'a str },
    Unknown(char),
    Dummy,
    Eof,
}

use TokenKind::*;

#[derive(Clone, Copy, Debug)]
struct Token<'a> {
    kind: TokenKind<'a>,
    span: Span,
}

impl<'a> Token<'a> {
    fn new(kind: TokenKind<'a>, span: Span) -> Self {
        Self { kind, span }
    }

    fn dummy() -> Self {
        Self::new(Dummy, Span::dummy())
    }

    fn glue(&self, _: &Self) -> Option<Self> {
        None
    }
}

#[derive(Debug)]
struct Lexer<'a> {
    input: Chars<'a>,
    pos: usize,
    token: Token<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Self {
            input: input.chars(),
            pos: 0,
            token: Token::dummy(),
        };
        let _ = lexer.bump();
        lexer
    }

    pub fn first(&self) -> Token<'a> {
        self.token
    }

    pub fn bump(&mut self) -> Token<'a> {
        let next_tok = loop {
            let (next_tok, is_next_tok_preceded_by_ws) = self.next_token_from_input();

            if is_next_tok_preceded_by_ws {
                break next_tok;
            } else if let Some(glued) = self.token.glue(&next_tok) {
                self.token = glued;
            } else {
                break next_tok;
            }
        };
        std::mem::replace(&mut self.token, next_tok)
    }

    fn next_token_from_input(&mut self) -> (Token<'a>, bool) {
        let mut preceeded_by_ws = false;
        loop {
            let str_before = self.input.as_str();
            let start = self.pos;
            let Some(first_char) = self.bump_char() else {
                return (Token::new(Eof, self.mk_sp(start)), preceeded_by_ws);
            };

            let kind = match first_char {
                c if is_whitespace(c) => {
                    self.ws();
                    preceeded_by_ws = true;
                    continue;
                }
                '0'..='9' => self.int(str_before, start),
                '-' => Minus,
                '+' => Plus,
                '/' => Slash,
                '*' => Star,
                '(' => LParen,
                ')' => RParen,
                _ => Unknown(first_char),
            };

            let span = self.mk_sp(start);
            return (Token::new(kind, span), preceeded_by_ws);
        }
    }

    fn ws(&mut self) {
        self.eat_char_while(is_whitespace)
    }

    fn int(&mut self, str_before: &'a str, start: usize) -> TokenKind<'a> {
        self.eat_dec_digits();
        let end = self.pos - start;
        Int {
            value: &str_before[..end],
        }
    }

    fn eat_dec_digits(&mut self) {
        loop {
            match self.first_char() {
                '0'..='9' | '_' => {
                    self.bump_char();
                }
                _ => break,
            }
        }
    }

    fn mk_sp(&self, start: usize) -> Span {
        Span::new(start, self.pos)
    }

    fn eat_char_while(&mut self, predicate: impl Fn(char) -> bool) {
        while !self.is_at_eof() && predicate(self.first_char()) {
            self.bump_char();
        }
    }

    fn bump_char(&mut self) -> Option<char> {
        let c = self.input.next();
        self.pos += c.map(|c| c.len_utf8()).unwrap_or(0);
        c
    }

    fn first_char(&self) -> char {
        self.input.clone().next().unwrap_or(EOF_CHAR)
    }

    fn is_at_eof(&self) -> bool {
        self.input.as_str().is_empty()
    }
}

const EOF_CHAR: char = '\0';

fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        | '\u{0020}' // space
        | '\u{0009}' // tab
        | '\u{000C}' // form feed
    )
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let t = self.bump();
        match t.kind {
            Eof => None,
            _ => Some(t),
        }
    }
}
