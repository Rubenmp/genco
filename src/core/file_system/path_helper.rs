use std::path::Path;

pub fn to_absolute_path_str(file: &Path) -> String {
    if !file.exists() {
        return file.to_string_lossy().to_string();
    }
    file.canonicalize().unwrap().to_string_lossy().to_string()
}
