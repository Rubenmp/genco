use std::fs::{File, OpenOptions};
use std::io::{Seek, Write};
use std::path::{Path, PathBuf};
use std::vec::Vec;
use std::{fs, io};

use crate::core::file_system::file_reader::get_number_of_bytes_of;
use crate::core::file_system::path_helper::try_to_absolute_path;

pub(crate) fn create_or_replace_file_with_bytes(
    output_file: &Path,
    bytes: &[u8],
) -> Result<(), String> {
    remove_file_if_exists(output_file)?;
    create_file_if_not_exist(output_file)?;

    replace_bytes_in_existing_file(output_file, bytes)
}

pub(in crate::core::file_system) fn replace_bytes_in_existing_file(
    output_file: &Path,
    bytes: &[u8],
) -> Result<(), String> {
    fs::remove_file(output_file).map_err(|e| e.to_string())?;
    let mut file = create_file_to_write(output_file)?;

    file.seek(io::SeekFrom::Start(0)).map_err(|e| {
        format!(
            "Can not seek start of file to write ({}):\n{}\n",
            e,
            try_to_absolute_path(output_file)
        )
    })?;

    file.write_all(bytes).map_err(|e| {
        format!(
            "Can not write to file ({}):\n{}\n",
            e,
            try_to_absolute_path(output_file)
        )
    })?;

    Ok(())
}

pub(crate) fn remove_file_if_exists(file_path: &Path) -> Result<(), String> {
    if file_path.exists() && file_path.is_file() {
        fs::remove_file(file_path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn create_file_if_not_exist(input_file: &Path) -> Result<(), String> {
    if input_file.exists() {
        return Ok(());
    }

    let all_to_create = get_all_paths_to_create(input_file);
    let last_path_index = all_to_create.len() - 1;
    for (i, to_create) in all_to_create.iter().rev().enumerate() {
        if i < last_path_index {
            fs::create_dir(to_create).map_err(|e| e.to_string())?;
        } else {
            File::create(to_create).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

pub(crate) fn copy(input_file: &Path, output_file: &Path) -> Result<(), String> {
    let data = fs::read(input_file).map_err(|_| {
        format!(
            "Error reading resource to get content from {:?}",
            input_file
        )
    })?;
    remove_file_if_exists(output_file)?;
    create_file_if_not_exist(output_file)?;

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(output_file)
        .expect("File can not be opened to write");
    let number_of_bytes = get_number_of_bytes_of(input_file);
    let mut buffer = vec![0; number_of_bytes];
    buffer[0..number_of_bytes].clone_from_slice(&data.to_vec());

    write_buffer(&mut file, &mut buffer);
    Ok(())
}

pub(crate) fn create_empty_file_if_not_exist_with_ancestor(file: &Path) -> Result<(), String> {
    if file.exists() {
        return Ok(());
    }

    create_ancestor_dirs(file)?;
    create_non_existent_file_with_content(file, &Vec::new())
}

/// This method assumes that previous directories are already created
pub(crate) fn create_non_existent_file_with_content(
    output_file: &Path,
    data: &[u8],
) -> Result<(), String> {
    let mut file = create_file_to_write(output_file)?;
    write_buffer_with_result(&mut file, data)?;

    Ok(())
}

fn create_file_to_write(file: &Path) -> Result<File, String> {
    match OpenOptions::new().write(true).create(true).open(file) {
        Ok(file) => Ok(file),
        Err(err) => Err(format!(
            "File can not be opened to write ({}):\n\"{}\"\n",
            err,
            try_to_absolute_path(file)
        )),
    }
}

/// This method returns the highest created directory, if any
pub(in crate::core::file_system) fn create_ancestor_dirs(
    input_file: &Path,
) -> Result<Option<PathBuf>, String> {
    let all_to_create = get_all_paths_to_create(input_file);
    let last_path_index = all_to_create.len() - 1;
    let mut highest_created_dir_opt = None;
    for (i, dir_to_create) in all_to_create.iter().rev().enumerate() {
        if i < last_path_index && !dir_to_create.exists() {
            fs::create_dir(dir_to_create).map_err(|e| e.to_string())?;
            if highest_created_dir_opt.is_none() {
                highest_created_dir_opt = Some(dir_to_create.clone())
            }
        }
    }

    Ok(highest_created_dir_opt)
}

fn get_all_paths_to_create(file_path: &Path) -> Vec<PathBuf> {
    let mut mut_file_path = file_path.to_path_buf();
    let mut all_to_create = Vec::new();

    for ancestor in file_path.ancestors() {
        if mut_file_path.exists() {
            break;
        } else {
            all_to_create.push(PathBuf::from(ancestor));
            mut_file_path.pop();
        }
    }
    all_to_create
}

fn write_buffer(file: &mut File, buffer: &mut [u8]) {
    file.seek(io::SeekFrom::Start(0))
        .expect("Seek resource to the beginning");
    file.write_all(buffer).expect("Write resource failed.");
}

fn write_buffer_with_result(file: &mut File, buffer: &[u8]) -> Result<(), String> {
    file.seek(io::SeekFrom::Start(0))
        .map_err(|e| e.to_string())?;
    file.write_all(buffer).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use crate::core::file_system::file_edition::file_editor::{copy, create_file_if_not_exist};
    use crate::core::testing::test_assert::assert_same_file;
    use crate::core::testing::test_path::{get_test_dir_raw, get_test_file};

    #[test]
    fn create_if_not_exists_test() {
        let current_file_path = &get_current_file_path();
        let mut file_path = get_test_dir_raw(current_file_path);
        file_path.push("new_folder");
        let file_path_copy = file_path.clone();
        file_path.push("new_file.rs");

        create_file_if_not_exist(&file_path).expect("Result file should be removed");

        assert!(file_path.exists());
        fs::remove_dir_all(file_path_copy.as_path())
            .expect("Test must remove the created files & folders");
    }

    #[test]
    fn copy_test() {
        let input_file = get_test_file(&get_current_file_path(), "create_file_with_content.txt");
        let mut output_file = input_file.clone();
        output_file.pop();
        output_file.push("create_file_with_content_output.txt");
        let _ = fs::remove_file(&output_file);

        copy(&input_file, &output_file).expect("Copy must succeed");

        assert!(output_file.exists());
        assert_same_file(&input_file, &output_file);
        let _ = fs::remove_file(&output_file);
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
