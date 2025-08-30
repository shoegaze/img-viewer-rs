use std::env;
use std::error::Error;
use std::path::PathBuf;

use crate::app::App;

pub fn open_image_from_path(app: &mut App, path: PathBuf) -> Result<(), Box<dyn Error>> {
    let canonical_path = to_canonical_path(&path)?;

    app.queue_image_open(canonical_path)?;

    Ok(())
}

pub fn to_canonical_path(path: &PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    let current_dir = &env::current_dir()?;
    let full_path: PathBuf = [current_dir, path].iter().collect();
    let full_path = full_path.canonicalize()?;

    Ok(full_path)
}
