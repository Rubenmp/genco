use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub(crate) fn read_dir(path: &Path) -> Vec<PathBuf> {
    if !path.exists() || !path.is_dir() {
        panic!("Error: expecting directory in {:?}", path);
    }
    let mut result = Vec::new();
    let paths_result = fs::read_dir(path).unwrap();

    for dir_entry_result in paths_result {
        match dir_entry_result {
            Ok(dir_entry) => result.push(dir_entry.path()),
            Err(_e) => panic!("Error get_dir_ending_with"),
        }
    }

    result
}

pub(crate) fn get_dir_of_file(input_path: &Path) -> PathBuf {
    let mut path = input_path.to_path_buf();
    path.pop();
    path.to_owned()
}

fn get_dir_ending_with(input_path: &Path, ending: &str) -> Option<PathBuf> {
    let paths_result = fs::read_dir(input_path).unwrap();

    for dir_entry_result in paths_result {
        match dir_entry_result {
            Ok(dir_entry) => {
                let path = dir_entry.path();
                if path.is_dir() && path.to_string_lossy().ends_with(ending) {
                    return Some(path);
                }
            }
            Err(_e) => panic!("Error get_dir_ending_with"),
        }
    }

    None
}

fn get_dir_map(path: &Path) -> HashMap<String, PathBuf> {
    let mut result = HashMap::new();
    for path in read_dir(path) {
        if path.is_dir() && path.exists() {
            if let Some(last_dir) = path.iter().last() {
                result.insert(last_dir.to_string_lossy().to_string(), path);
            }
        }
    }

    result
}

fn get_dir(input_dir: &Path, dir_name: &str) -> Option<PathBuf> {
    for path in read_dir(input_dir) {
        if path.is_dir() {
            if let Some(found_dir_name) = path.iter().last() {
                if dir_name.eq(&found_dir_name.to_string_lossy()) {
                    return Some(path);
                }
            }
        }
    }

    None
}

fn check_dir_exist(input_dir: &Path, start_error_message: &str) {
    if !input_dir.exists() || !input_dir.is_dir() {
        panic!("{}: {:?}", start_error_message, input_dir)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::file_system::directory_browsing::directory_browser::{
        check_dir_exist, get_dir, get_dir_ending_with, get_dir_map,
    };
    use crate::core::testing::test_assert::{assert_dir_is, assert_fail};
    use crate::core::testing::test_path::get_test_dir;

    #[test]
    fn get_dir_ending_in_boot() {
        let test_dir = get_test_dir(get_current_file_path(), "get_dir_ending_in_boot");
        let mut expected_test_dir = test_dir.clone();
        expected_test_dir.push("app-boot");

        let boot_dir = get_dir_ending_with(&test_dir, "boot");

        if let Some(dir) = boot_dir {
            assert_eq!(expected_test_dir.to_string_lossy(), dir.to_string_lossy());
        } else {
            assert_fail("boot_dir not found");
        }
    }

    #[test]
    fn get_dir_code() {
        let test_dir = get_test_dir(get_current_file_path(), "get_dir");
        let mut expected_test_dir = test_dir.clone();
        expected_test_dir.push("code");

        let boot_dir = get_dir(&test_dir, "code");

        if let Some(dir) = boot_dir {
            assert_eq!(expected_test_dir.to_string_lossy(), dir.to_string_lossy());
        } else {
            assert_fail("code directory not found");
        }
    }

    #[test]
    fn get_dir_map_dir_with_files() {
        let test_dir = get_test_dir(get_current_file_path(), "get_dir_map");
        let mut expected_test_dir = test_dir.clone();
        expected_test_dir.push("code");

        let dir_map = get_dir_map(&test_dir);

        assert_eq!(2, dir_map.len());
        if let Some(first_dir) = dir_map.get("first_dir") {
            assert_dir_is(first_dir, "first_dir");
        } else {
            assert_fail("Expected dir_map entry \"first_dir\" not found.");
        }
        if let Some(first_dir) = dir_map.get("second_dir") {
            assert_dir_is(first_dir, "second_dir");
        } else {
            assert_fail("Expected dir_map entry \"second_dir\" not found.");
        }
    }

    #[test]
    fn check_dir_test() {
        let test_dir = get_test_dir(get_current_file_path(), "get_dir_map");

        check_dir_exist(&test_dir, "Error");
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
