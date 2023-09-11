use std::fs;
use std::path::{Path, PathBuf};

use crate::core::database::model::java_import_route::db_java_import_route_save;
use crate::core::database::model::java_import_route::java_import_route::JavaImportRouteCreate;
use crate::core::file_system::path_helper::to_absolute_path_str;
use crate::java::scanner::package::java_package_scanner;

pub fn scan(base_java_path: &Path) -> Result<(), String> {
    java_package_scanner::check_base_java_project(base_java_path);
    recursive_scan(base_java_path);

    Ok(())
}

fn recursive_scan(path: &Path) {
    let files_and_dirs = get_files_and_dirs_to_scan(path);
    insert_java_import_routes_in_db(files_and_dirs.0);

    for dir in files_and_dirs.1 {
        recursive_scan(&dir);
    }
}

fn insert_java_import_routes_in_db(java_files: Vec<PathBuf>) {
    if java_files.is_empty() {
        return;
    }

    let routes_to_save: Vec<JavaImportRouteCreate> = java_files
        .iter()
        .map(|file| JavaImportRouteCreate::new(file))
        .collect();

    db_java_import_route_save::save(routes_to_save)
        .expect("JavaImportRoute batch save must succeed")
}

fn get_files_and_dirs_to_scan(path: &Path) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let mut files = Vec::new();
    let mut dirs = Vec::new();
    let paths_result = fs::read_dir(path).unwrap_or_else(|_| {
        panic!(
            "Error scanning directory:\n\"{}\"\n",
            to_absolute_path_str(path)
        )
    });

    for dir_entry_result in paths_result {
        match dir_entry_result {
            Ok(dir_entry) => {
                let path = dir_entry.path();
                if path.is_file() {
                    if is_java_file(&path) {
                        files.push(path);
                    }
                } else if path.is_dir() && java_package_scanner::should_scan_dir(&path) {
                    dirs.push(path);
                }
            }
            Err(_e) => panic!("Error get_files_and_dirs_to_scan"),
        }
    }

    (files, dirs)
}

fn is_java_file(file_name: &PathBuf) -> bool {
    match file_name.extension() {
        None => false,
        Some(extension) => extension.eq("java"),
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::database::model::java_import_route::db_java_import_route_search;
    use crate::core::testing::test_path::get_test_dir;
    use crate::java::scanner::package::java_dependency_scanner;

    #[test]
    fn scan_java_project_test() {
        let dir_path = get_test_dir(get_current_file_path(), "basic_project");

        let scan_result = java_dependency_scanner::scan(&dir_path);

        scan_result.expect("Scan must be ok");
        let n_demo_app = db_java_import_route_search::by_last_type_id("DemoApplication").len();
        assert_eq!(1, n_demo_app);
    }

    #[test]
    fn get_files_and_dirs_to_scan_test() {
        let dir_path = get_test_dir(get_current_file_path(), "basic_project");

        let (files, dirs) = java_dependency_scanner::get_files_and_dirs_to_scan(&dir_path);

        assert_eq!(0, files.len());
        assert_eq!(1, dirs.len());
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
