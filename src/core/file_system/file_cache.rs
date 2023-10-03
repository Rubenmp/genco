use crate::core::file_system::file_reader;
use crate::core::parser::string_helper;
use std::path::{Path, PathBuf};

pub(crate) struct FileCache {
    path: PathBuf,
    content: Vec<u8>,
}

impl FileCache {
    pub(crate) fn from(path: &Path) -> Self {
        let bytes = file_reader::read_all_bytes(path).expect("Expected file cache to be readable");
        Self {
            path: path.to_path_buf(),
            content: bytes,
        }
    }

    pub(crate) fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub(crate) fn get_content(&self, start_byte: usize, end_byte: usize) -> String {
        let result_bytes = &self.content[start_byte..end_byte];
        string_helper::to_str(result_bytes)
    }
}
