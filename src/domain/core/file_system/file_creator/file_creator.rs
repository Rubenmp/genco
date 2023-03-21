use std::fs::{File, OpenOptions};
use std::io::{Seek, Write};
use std::path::{Path, PathBuf};
use std::vec::Vec;
use std::{fs, io};

use crate::domain::core::file_system::file_reader::get_number_of_bytes_of;

#[allow(unused)]
pub fn create_file_if_not_exist(file_path: &PathBuf) {
    if !file_path.exists() {
        let mut mut_file_path = file_path.clone();
        let mut all_to_create = Vec::new();

        for ancestor in file_path.ancestors() {
            if mut_file_path.exists() {
                break;
            } else {
                all_to_create.push(PathBuf::from(ancestor));
                mut_file_path.pop();
            }
        }

        for (i, to_create) in all_to_create.iter().rev().enumerate() {
            if i < (all_to_create.len() - 1) {
                fs::create_dir(to_create.clone());
            } else {
                File::create(to_create.clone());
            }
        }
    }
}

pub fn create_file_if_not_exists_with_content(output_file: &Path, content: &str) {
    if !output_file.exists() {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(output_file)
            .expect("File can not be opened to write");
        let content_vec_u8 = content.as_bytes();
        let number_of_bytes = content_vec_u8.len();
        let mut buffer = vec![0; number_of_bytes];
        buffer[0..number_of_bytes].clone_from_slice(&content_vec_u8.to_vec());

        write_buffer(&mut file, &mut buffer);
    }
}

pub fn create_file_with_content(output_file: &PathBuf, content_path: &PathBuf) {
    let _a = 0;
    let data = fs::read(content_path)
        .expect(format!("Error reading file to get content from {:?}", content_path).as_str());
    remove_file_if_exists(output_file);
    create_file_if_not_exist(output_file);

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(output_file)
        .expect("File can not be opened to write");
    let number_of_bytes = get_number_of_bytes_of(content_path);
    let mut buffer = vec![0; number_of_bytes];
    buffer[0..number_of_bytes].clone_from_slice(&data.to_vec());

    write_buffer(&mut file, &mut buffer);
}

fn write_buffer(file: &mut File, buffer: &mut Vec<u8>) {
    file.seek(io::SeekFrom::Start(0))
        .expect("Seek file to the beginning");
    file.write_all(&buffer).expect("Write file failed.");
}

pub fn remove_file_if_exists(file_path: &PathBuf) {
    if file_path.exists() && file_path.is_file() {
        fs::remove_file(file_path).expect("Error removing file");
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use crate::domain::core::file_system::file_creator::file_creator::{
        create_file_if_not_exist, create_file_with_content, remove_file_if_exists,
    };
    use crate::domain::core::testing::test_assert::assert_same_file;
    use crate::domain::core::testing::test_path::{get_test_dir_raw, get_test_file};

    #[test]
    fn create_if_not_exists_test() {
        let mut file_path = get_test_dir_raw(get_current_file_path());
        file_path.push("new_folder");
        let file_path_copy = file_path.clone();
        file_path.push("new_file.rs");

        create_file_if_not_exist(&file_path);

        assert!(file_path.exists());
        fs::remove_dir_all(file_path_copy.as_path())
            .expect("Test must remove the created files & folders");
    }

    #[test]
    fn create_file_with_content_test() {
        let input_file = get_test_file(get_current_file_path(), "create_file_with_content.txt");
        let mut output_file = input_file.clone();
        output_file.pop();
        output_file.push("create_file_with_content_output.txt");
        remove_file_if_exists(&output_file);

        create_file_with_content(&output_file, &input_file);

        assert!(output_file.exists());
        assert_same_file(&input_file, &output_file);
        remove_file_if_exists(&output_file);
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
