use std::path::Path;

pub fn to_absolute_path_str(file: &Path) -> String {
    if !file.exists() {
        return file.to_string_lossy().to_string();
    }
    file.canonicalize().unwrap().to_string_lossy().to_string()
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::file_system::path_helper::to_absolute_path_str;

    #[test]
    fn to_absolute_path_str_test() {
        let current_path = get_current_file_path();

        let result = to_absolute_path_str(&current_path);

        assert!(result.ends_with("genco/src/core/file_system/path_helper.rs"));
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}