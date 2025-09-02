use std::path::PathBuf;

// See image-rs supported file extensions:
//  https://github.com/image-rs/image?tab=readme-ov-file#supported-image-formats
static SUPPORTED_FILE_EXTENSIONS: [&str; 8] =
    ["bmp", "gif", "jpg", "jpeg", "png", "tga", "tiff", "webp"];

pub fn validate_path(path: &PathBuf) -> bool {
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
