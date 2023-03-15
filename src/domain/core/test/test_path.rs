#![allow(unused)]
// These methods are used just in test mode.
// The warnings are removed to prevent pollution of stderr in normal executions.

use std::path::PathBuf;

fn get_base_test_resource_path() -> PathBuf {
    let mut rsrc_dir = std::env::current_exe().expect("Can't find path to executable");
    rsrc_dir.pop();
    rsrc_dir.pop();
    rsrc_dir.pop();
    rsrc_dir.pop();
    rsrc_dir.push("resource");
    rsrc_dir.push("test");

    rsrc_dir
}

pub fn get_test_folder_path(current_dir: PathBuf) -> PathBuf {
    let mut path = get_base_test_resource_path().clone();

    for component in current_dir.components() {
        let component_str = component.as_os_str().to_str().unwrap();
        if component_str.ends_with(".rs") {
            let comp_removed_rs_extension: &str = &component_str[0..component_str.len() - 3];
            path.push(comp_removed_rs_extension);
        } else {
            path.push(component_str);
        }
    }
    path
}

// Java
pub fn get_java_test_file_path(current_dir: PathBuf, test_name: &str) -> PathBuf {
    let mut path = get_test_folder_path(current_dir);

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

pub fn get_test_file_path(current_dir: PathBuf, test_name: &str) -> PathBuf {
    let mut path = get_test_folder_path(current_dir);
    path.push(test_name);

    path
}
