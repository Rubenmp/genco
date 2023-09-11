use std::path::Path;

use crate::core::observability::logger::logger;
use crate::java::scanner::file::dto::java_file::JavaFile;
use crate::java::scanner::file::java_file_scanner;

pub struct JavaProjectFilesCache {
    files: Vec<JavaFile>,
}

impl JavaProjectFilesCache {
    pub fn new() -> Self {
        JavaProjectFilesCache { files: Vec::new() }
    }

    pub fn try_to_add_file(&mut self, file: &Path) {
        if !file.exists() || !file.is_file() {
            panic!(
                "Trying to add invalid resource to JavaProjectFilesCache: {:?}",
                file
            );
        }
        match java_file_scanner::scan(file) {
            Ok(java_file) => {
                self.add_file(java_file);
            }
            Err(e) => logger::log_warning(&e),
        }
    }

    fn add_file(&mut self, file: JavaFile) {
        self.files.push(file);
    }

    pub fn count_files_added(&self) -> usize {
        self.files.len()
    }
}
