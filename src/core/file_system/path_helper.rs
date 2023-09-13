use std::path::Path;

pub(crate) fn try_to_absolute_path(file: &Path) -> String {
    if !file.exists() {
        return file.to_string_lossy().to_string();
    }

    if let Ok(file_canonicalize) = file.canonicalize() {
        return file_canonicalize.to_string_lossy().to_string();
    }
    return file.to_string_lossy().to_string();
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::file_system::path_helper::try_to_absolute_path;

    #[test]
    fn to_absolute_path_str_test() {
        let current_path = get_current_file_path();

        let result = try_to_absolute_path(&current_path);

        assert!(result.ends_with("genco/src/core/file_system/path_helper.rs"));
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
