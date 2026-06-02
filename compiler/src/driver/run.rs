use std::path::PathBuf;

use crate::{
    ci::{CanaryConfig, run_ci},
    driver::CanaryDriver,
    ir::source::SourceFile,
    queries::parse_file_query,
};

impl CanaryDriver {
    pub fn run(self) {
        run_ci(CanaryConfig::from(self), |db| {
            let input_file_path = db.input();
            let input_file_contents =
                std::fs::read_to_string(input_file_path).expect("unable to read file"); // TODO: diagnostic
            let input_source_file =
                SourceFile::new(db, PathBuf::from(input_file_path), input_file_contents);

            let canary_file = parse_file_query::parse_file(db, input_source_file);
        });
    }
}
