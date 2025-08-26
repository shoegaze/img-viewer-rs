use clap::Parser;

use std::error::Error;
use std::path::PathBuf;

static VALID_FILE_EXTENSIONS: [&str; 1] = ["png"];

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    pub path: PathBuf,
}

impl Args {
    pub fn validated(self) -> Result<Self, Box<dyn Error>> {
        if !self.valid_path() {
            return Err(Box::from("Invalid image path"));
        }

        Ok(self)
    }

    fn valid_path(&self) -> bool {
        let path = &self.path;

        if path.is_dir() {
            return true;
        }

        if path.is_file() {
            let extension = path.extension().unwrap();

            return VALID_FILE_EXTENSIONS.iter().any(|&ext| extension == ext);
        }

        false
    }
}
