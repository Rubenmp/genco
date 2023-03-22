use std::path::{Path, PathBuf};

pub struct JavaProjectFilesCache {
    files: Vec<PathBuf>,
}

impl JavaProjectFilesCache {
    pub fn new() -> Self {
        JavaProjectFilesCache { files: Vec::new() }
    }

    pub fn add_file(&mut self, file: &Path) {
        if !file.exists() || !file.is_file() {
            panic!(
                "Trying to add invalid file to JavaProjectFilesCache: {:?}",
                file
            );
        }

        self.files.push(file.to_path_buf());
    }

    pub fn count_files_added(&self) -> usize {
        self.files.len()
    }
}
