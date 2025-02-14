use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::vec::Vec;

use crate::core::file_system::directory_browsing::directory_browser;
use crate::core::observability::logger;

#[allow(unused)]
pub fn get_first_file_from_dir_if_exists(path: &Path, filenames: Vec<&str>) -> Option<PathBuf> {
    if !path.exists() || !path.is_dir() {
        logger::log_warning(
            format!(
                "Function \"get_first_file_if_exists\" requires a directory, found: {:?}",
                path
            )
            .as_str(),
        );
        return None;
    }
    let paths_result = fs::read_dir(path).expect("Not able to read dir");

    for dir_entry_result in paths_result {
        match dir_entry_result {
            Ok(dir_entry) => {
                let path = dir_entry.path();
                if path.is_file() {
                    if let Some(found_file) = path.iter().last() {
                        if let Some(found_file_str) = found_file.to_str() {
                            if filenames.contains(&found_file_str) {
                                return Some(path);
                            }
                        }
                    }
                }
            }
            Err(_e) => println!("Error get_dir_ending_with"),
        }
    }

    None
}

pub fn get_file_map(path: &Path) -> HashMap<String, PathBuf> {
    let mut result = HashMap::new();
    for path in directory_browser::read_dir(path) {
        if path.is_file() && path.exists() {
            if let Some(last_dir) = path.iter().last() {
                result.insert(last_dir.to_string_lossy().to_string(), path);
            }
        }
    }

    result
}

pub fn do_last_element_in_path_ends_with(path: &Path, ending: &str) -> bool {
    let file_name = path
        .iter()
        .last()
        .expect("Last item in path must exists")
        .to_string_lossy()
        .to_string();
    file_name.ends_with(ending)
}

pub(crate) fn remove_java_extension(java_file_name: String) -> String {
    let until = java_file_name.len() - 5; // Remove ".java" extension
    java_file_name.clone().drain(0..until).as_str().to_string()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::file_system::file_browsing::file_browser::{
        get_file_map, get_first_file_from_dir_if_exists,
    };
    use crate::core::testing::test_assert::{assert_fail, assert_file_is};
    use crate::core::testing::test_path::get_test_dir;

    #[test]
    fn get_first_file_if_exists_java_build_files_find_pom() {
        let dir_path = get_test_dir(get_current_file_path(), "get_first_file_if_exists_java_pom");
        let mut files = Vec::new();
        files.push("build.gradle");
        files.push("pom.xml");

        let pom_opt = get_first_file_from_dir_if_exists(&dir_path, files);

        if let Some(pom) = pom_opt {
            assert_file_is(&pom, "pom.xml");
        } else {
            assert_fail("pom.xml resource not found");
        }
    }

    #[test]
    fn get_first_file_if_exists_java_build_files_not_find() {
        let dir_path = get_test_dir(get_current_file_path(), "get_first_file_if_exists_java_pom");
        let mut files = Vec::new();
        files.push("build.gradle");

        let pom_opt = get_first_file_from_dir_if_exists(&dir_path, files);

        assert!(pom_opt.is_none());
    }

    #[test]
    fn get_file_map_test() {
        let dir_path = get_test_dir(get_current_file_path(), "get_file_map");

        let file_map = get_file_map(&dir_path);

        assert_eq!(2, file_map.len());
        if let Some(gitignore) = file_map.get(".gitignore") {
            assert_file_is(gitignore, ".gitignore");
        } else {
            assert_fail("Expected \".gitignore\" resource not found.");
        }
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
