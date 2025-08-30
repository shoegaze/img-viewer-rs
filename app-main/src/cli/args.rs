use clap::Parser;

use std::error::Error;
use std::path::PathBuf;

// See image-rs supported file extensions:
//  https://github.com/image-rs/image?tab=readme-ov-file#supported-image-formats
static SUPPORTED_FILE_EXTENSIONS: [&str; 8] =
    ["bmp", "gif", "jpg", "jpeg", "png", "tga", "tiff", "webp"];

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
        let Some(extension) = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_string().to_lowercase())
        else {
            return false;
        };

        return SUPPORTED_FILE_EXTENSIONS
            .iter()
            .any(|&ext| extension == ext);
    }

    false
}
