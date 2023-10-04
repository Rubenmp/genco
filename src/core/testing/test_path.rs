#![allow(unused)]
// These methods are used just in test mode.
// The warnings are removed to prevent pollution of stderr in normal executions.

use std::path::{Path, PathBuf};

pub(crate) fn get_test_dir(current_file: PathBuf, name: &str) -> PathBuf {
    let mut path = get_test_dir_raw(current_file);
    path.push(name);

    if !path.exists() {
        panic!("Expected test directory does not exists: {:?}", path);
    } else if !path.is_dir() {
        panic!("Expected test directory is not a directory: {:?}", path);
    }
    path
}

pub(crate) fn get_test_dir_raw(current_file: PathBuf) -> PathBuf {
    let mut path = current_file;
    path.pop();
    path.push("test");
    path
}

pub(crate) fn get_test_file(current_file: PathBuf, name: &str) -> PathBuf {
    let path = get_non_existing_test_file(current_file, name);

    if !path.exists() {
        panic!("Expected test resource does not exists: {:?}", path);
    } else if !path.is_file() {
        panic!("Expected test resource is not a resource: {:?}", path);
    }
    path
}

pub(crate) fn get_non_existing_test_file(current_file: PathBuf, name: &str) -> PathBuf {
    let mut path = current_file;
    path.pop();
    path.join("test").join(name)
}

// Java
pub(crate) fn get_java_test_file(
    current_file: PathBuf,
    test_folder: &str,
    java_file_name: &str,
) -> PathBuf {
    let mut path = get_java_project_test_folder(current_file, test_folder);
    path.join(java_file_name)
}

pub(crate) fn get_java_project_test_folder(current_file: PathBuf, test_folder: &str) -> PathBuf {
    let mut path = get_test_dir_raw(current_file);

    include_path_to_main_java_folder(&mut path, test_folder);

    path
}

fn include_path_to_main_java_folder(path: &mut PathBuf, test_folder: &str) {
    path.push(test_folder);
    path.push("src");
    path.push("main");
    path.push("java");
    path.push("org");
    path.push("test");
}
