use std::fs;
use std::path::{Path, PathBuf};

use crate::core::file_system::file_edition::file_editor;
use crate::core::file_system::file_overwriting::file_overwriter::FileOverwriting;
use crate::core::file_system::file_reader;
use crate::core::file_system::path_helper::try_to_absolute_path;

/// This structure hides the complexity of file system modifications.
/// If a method returns an error it should not have any effect on the filesystem.
/// This buffer stores created directories and files to remove them altogether
/// if there is an unexpected error.
///
/// Methods from core/file_system that edits files should not be visible directly but
/// using this buffer.
pub(crate) struct FileSystemEditorBuffer {
    created_dirs: Vec<PathBuf>,
    created_files: Vec<PathBuf>,
    edited_files: Vec<(PathBuf, Vec<u8>)>,
}

// Public crate methods
impl FileSystemEditorBuffer {
    pub(crate) fn new() -> Self {
        Self {
            created_dirs: vec![],
            created_files: vec![],
            edited_files: vec![],
        }
    }

    /// # write_all_to_file
    /// Override output_file content, rolling back all previous operations
    /// attached to the buffer if there is any error.
    /// Modifications to the file system associated to this buffer
    /// are rolled back if there is an error.
    pub(crate) fn write_all_to_file(
        &mut self,
        file_overwriting: &mut FileOverwriting,
        output_file: &Path,
    ) -> Result<(), String> {
        match self.write_all_to_file_internal(file_overwriting, output_file) {
            Ok(_) => Ok(()),
            Err(err) => {
                self.rollback();
                Err(err)
            }
        }
    }

    /// # write_all
    /// Override file_overwriting file content, rolling back all previous operations
    /// attached to the buffer if there is any error.
    /// Modifications to the file system associated to this buffer
    /// are rolled back if there is an error.
    pub(crate) fn write_all(
        &mut self,
        file_overwriting: &mut FileOverwriting,
    ) -> Result<(), String> {
        let output_file = file_overwriting.get_input_file().to_owned();
        self.write_all_to_file(file_overwriting, &output_file)
    }

    pub(crate) fn create_empty_file_if_not_exist(&mut self, file: &Path) -> Result<(), String> {
        self.create_empty_file_if_not_exist_internal(file)
    }

    pub(crate) fn write_content_to_file(
        &mut self,
        file: &Path,
        result_buffer: Vec<u8>,
    ) -> Result<(), String> {
        match self.write_content_to_file_internal(file, result_buffer) {
            Ok(_) => Ok(()),
            Err(err) => {
                self.rollback();
                Err(err)
            }
        }
    }
}

// Private methods
impl FileSystemEditorBuffer {
    fn write_all_to_file_internal(
        &mut self,
        file_overwriting: &mut FileOverwriting,
        output_file: &Path,
    ) -> Result<(), String> {
        let result_buffer = file_overwriting.get_written_buffer()?;
        self.write_content_to_file_internal(output_file, result_buffer)
    }

    fn create_empty_file_if_not_exist_internal(&mut self, file: &Path) -> Result<(), String> {
        if file.exists() {
            return Ok(());
        }

        self.create_ancestor_dirs_for_file(file)?;
        file_editor::create_non_existent_file_with_content(file, &Vec::new())
    }

    fn write_content_to_file_internal(
        &mut self,
        file: &Path,
        result_buffer: Vec<u8>,
    ) -> Result<(), String> {
        let file_exists = file.exists();
        if file_exists && file.is_dir() {
            return Err(format!(
                "Attempt to override a file that it is a directory:\n{}\n",
                try_to_absolute_path(file)
            ));
        }

        if file_exists {
            let current_file_bytes = file_reader::read_all_bytes(file)?;
            file_editor::replace_bytes_in_existing_file(file, &result_buffer)?;
            self.edited_files
                .push((file.to_path_buf(), current_file_bytes));
        } else {
            self.create_ancestor_dirs_for_file(file)?;
            file_editor::create_non_existent_file_with_content(file, &result_buffer)?;
        }

        Ok(())
    }

    fn create_ancestor_dirs_for_file(&mut self, file: &Path) -> Result<(), String> {
        let highest_created_dir_opt = file_editor::create_ancestor_dirs(file)?;
        highest_created_dir_opt
            .iter()
            .for_each(|created_dir| self.created_dirs.push(created_dir.to_owned()));

        Ok(())
    }

    fn rollback(&mut self) {
        self.remove_created_directories_silently();
        self.remove_created_files_silently();
        self.rollback_edited_files_silently();
    }

    fn remove_created_directories_silently(&mut self) {
        for created_dir in &self.created_dirs {
            let _ = fs::remove_dir_all(created_dir);
        }

        self.created_dirs.clear();
    }

    fn remove_created_files_silently(&mut self) {
        for created_file in &self.created_files {
            let _ = file_editor::remove_file_if_exists(created_file);
        }

        self.created_files.clear();
    }

    fn rollback_edited_files_silently(&mut self) {
        for edited_file_pair in &self.edited_files {
            let edited_file = edited_file_pair.0.to_owned();
            let previous_content_in_bytes = edited_file_pair.1.to_owned();
            let _ = file_editor::replace_bytes_in_existing_file(
                &edited_file,
                &previous_content_in_bytes,
            );
        }

        self.edited_files.clear();
    }
}
