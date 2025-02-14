use std::fs;
use std::path::{Path, PathBuf};

use crate::core::database::model::java_import_route::java_import_route_entity::{
    JavaImportRouteCreate, JavaImportRouteEntity,
};
use crate::core::database::model::java_import_route::{
    db_java_import_route_save, db_java_import_route_search,
};

use crate::core::file_system::path_helper::try_to_absolute_path;
use crate::java::scanner::package::java_package_scanner;

/// TODO: optimize this, do not scan previously scanned dirs
/// - Current approach: rescan always
/// - Is it possible to detect any change within a directory to avoid rescan?
pub(crate) fn recursive_scan_dir_unchecked(base_java_project_dir: &Path) {
    recursive_scan(base_java_project_dir);
}

///
/// WARNING: this method only return imports in the same java project that "java_file_containing_route".
/// Multi-module support is not yet implemented. Input parameters example:
///
/// - import_route -> "org.test.JavaClassFrom"
///
/// - java_file_containing_route -> any valid java file in a project containing "import <import_route>;"
pub(crate) fn search_imports(import_route: &str, java_file: &Path) -> Vec<JavaImportRouteEntity> {
    let base_package_path_opt = java_package_scanner::get_base_package(java_file);
    if let Some(base_package_path) = base_package_path_opt {
        return db_java_import_route_search::by_base_package_and_route(
            &base_package_path,
            import_route,
        );
    }

    vec![]
}

fn recursive_scan(path: &Path) {
    let files_and_dirs = get_files_and_dirs_to_scan(path);
    insert_java_import_routes_in_db(files_and_dirs.0);

    for dir in files_and_dirs.1 {
        recursive_scan(&dir);
    }
}

fn insert_java_import_routes_in_db(java_files: Vec<PathBuf>) {
    let routes_to_save: Vec<JavaImportRouteCreate> = JavaImportRouteCreate::from(java_files);

    db_java_import_route_save::save(routes_to_save)
        .expect("JavaImportRoute batch save must succeed")
}

fn get_files_and_dirs_to_scan(path: &Path) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let mut files = Vec::new();
    let mut dirs = Vec::new();
    let paths_result = fs::read_dir(path).unwrap_or_else(|_| {
        panic!(
            "Error scanning directory:\n\"{}\"\n",
            try_to_absolute_path(path)
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

fn is_java_file(file_name: &Path) -> bool {
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
        let dir_path = get_local_test_dir().join("basic_project");

        java_dependency_scanner::recursive_scan_dir_unchecked(&dir_path);

        let result_imports = db_java_import_route_search::by_last_type_id("DemoApplication");
        assert_eq!(1, result_imports.len());
        if let Some(result_import) = result_imports.get(0) {
            assert_eq!("DemoApplication", result_import.get_last_type_id());
            assert_eq!("org.test.DemoApplication", result_import.get_route());
            assert!(result_import.get_base_package().ends_with(
                "genco/src/java/scanner/package/test/java_dependency_scanner/basic_project"
            ));
        }
    }

    #[test]
    fn get_files_and_dirs_to_scan_test() {
        let dir_path = get_local_test_dir().join("basic_project");

        let (files, dirs) = java_dependency_scanner::get_files_and_dirs_to_scan(&dir_path);

        assert_eq!(0, files.len());
        assert_eq!(1, dirs.len());
    }

    fn get_local_test_dir() -> PathBuf {
        get_test_dir(get_current_file_path(), "java_dependency_scanner")
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
