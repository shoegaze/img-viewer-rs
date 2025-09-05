use clap::Parser;

use std::path::PathBuf;

use crate::cli::validate::{validate_dir, validate_path};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub dir: Option<PathBuf>,

    #[arg(
        value_name = "FILES",
        value_parser = clap::value_parser!(PathBuf),
        value_hint = clap::ValueHint::FilePath
    )]
    pub paths: Vec<PathBuf>,
}

pub struct ValidatedArgs {
    pub paths: Vec<PathBuf>,
}

impl Args {
    pub fn validated(self) -> ValidatedArgs {
        let mut valid_paths: Vec<PathBuf> = vec![];

        {
            // Validate arg: `dir`
            if let Some(dir) = self.dir {
                valid_paths.append(&mut validate_dir(dir));
            }
        }

        {
            // Validate arg: `...FILES`
            let mut paths = self
                .paths
                .iter()
                .map(|path| path.to_owned())
                .filter_map(|path| validate_path(path))
                .collect();

            valid_paths.append(&mut paths);
        }

        ValidatedArgs { paths: valid_paths }
    }
}
