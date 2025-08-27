use clap::Parser;

use std::error::Error;
use std::path::PathBuf;

static VALID_FILE_EXTENSIONS: [&str; 1] = ["png"];

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(
        value_name = "FILES",
        value_parser = clap::value_parser!(PathBuf),
        value_hint = clap::ValueHint::FilePath
    )]
    pub paths: Vec<PathBuf>,
}

impl Args {
    pub fn validated(self) -> Result<Self, Box<dyn Error>> {
        let mut valid_paths: Vec<PathBuf> = vec![];

        for path in self.paths {
            let is_valid_path = validate_path(&path);

            if !is_valid_path {
                eprintln!("Path is not a supported image: '{}'", path.display());
                continue;
            }

            valid_paths.push(path);
        }

        let validated_args = Args { paths: valid_paths };

        Ok(validated_args)
    }
}

fn validate_path(path: &PathBuf) -> bool {
    if path.is_dir() {
        // TODO: Recursively check files in dir
        return true;
    }

    if path.is_file() {
        let extension = path.extension().unwrap();

        return VALID_FILE_EXTENSIONS.iter().any(|&ext| extension == ext);
    }

    false
}
