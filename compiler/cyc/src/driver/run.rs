use std::path::PathBuf;

use crate::{
    ci::{CanaryConfig, run_ci},
    driver::CanaryDriver,
};

impl CanaryDriver {
    pub fn run(self) {
        run_ci(CanaryConfig::from(self), |ci| {
            let input_file = ci.source_map.add(&ci.cfg.input);
            // let input_file_path = db.input();
            // let input_file_contents =
            //     std::fs::read_to_string(input_file_path).expect("unable to read file"); // TODO: diagnostic
            // let input_source_file =
            //     SourceFile::new(db, PathBuf::from(input_file_path), input_file_contents);
        });
    }
}
