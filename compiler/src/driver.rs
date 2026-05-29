use std::{
    fmt::Display,
    fs,
    io::{self, Write},
    path::PathBuf,
    str::FromStr,
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
                Ok(0) | Err(_) => break,
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
        print!("{src}")
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
