#![allow(unused)]
// These methods are used just in test mode.
// The warnings are removed to prevent pollution of stderr in normal executions.

use std::path::{Path, PathBuf};

pub fn get_test_dir(current_file: PathBuf, name: &str) -> PathBuf {
    let mut path = get_test_dir_raw(current_file);
    path.push(name);

    if !path.exists() {
        panic!("Expected test directory does not exists: {:?}", path);
    } else if !path.is_dir() {
        panic!("Expected test directory is not a directory: {:?}", path);
    }
    path
}

pub fn get_test_dir_raw(current_file: PathBuf) -> PathBuf {
    let mut path = current_file;
    path.pop();
    path.push("test");
    path
}

pub fn get_test_file(current_file: PathBuf, name: &str) -> PathBuf {
    let path = get_non_existing_test_file(current_file, name);

    if !path.exists() {
        panic!("Expected test resource does not exists: {:?}", path);
    } else if !path.is_file() {
        panic!("Expected test resource is not a resource: {:?}", path);
    }
    path
}

pub fn get_non_existing_test_file(current_file: PathBuf, name: &str) -> PathBuf {
    let mut path = current_file;
    path.pop();
    path.join("test").join(name)
}

// Java
pub fn get_java_test_file(current_dir: PathBuf, test_name: &str) -> PathBuf {
    let mut path = get_test_dir_raw(current_dir);

    include_path_to_main_java_file(test_name, &mut path);
    path
}

fn include_path_to_main_java_file(test_name: &str, path: &mut PathBuf) {
    path.push(test_name);
    path.push("src");
    path.push("main");
    path.push("java");
    path.push("org");
    path.push("gencotest");
    path.push("Main.java");
}
