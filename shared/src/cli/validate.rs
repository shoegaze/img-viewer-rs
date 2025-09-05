use std::path::PathBuf;

// See image-rs supported file extensions:
//  https://github.com/image-rs/image?tab=readme-ov-file#supported-image-formats
static SUPPORTED_FILE_EXTENSIONS: [&str; 8] =
    ["bmp", "gif", "jpg", "jpeg", "png", "tga", "tiff", "webp"];

pub fn validate_path(path: PathBuf) -> Option<PathBuf> {
    if !path.is_file() {
        return None;
    }

    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_string().to_lowercase())?;

    let is_ext_supported = SUPPORTED_FILE_EXTENSIONS
        .iter()
        .any(|&ext| extension == ext);

    if is_ext_supported { Some(path) } else { None }
}

pub fn validate_dir(path: PathBuf) -> Vec<PathBuf> {
    if !path.is_dir() {
        return vec![];
    }

    let Ok(paths) = path.read_dir() else {
        return vec![];
    };

    paths
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter_map(|path| validate_path(path))
        .collect()
}
