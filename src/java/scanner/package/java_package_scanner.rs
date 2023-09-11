use std::path::{Path, PathBuf};

use crate::core::file_system::directory_browser::directory_browser;
use crate::core::file_system::file_browser::file_browser;
use crate::core::file_system::path_helper;
use crate::core::observability::logger::logger::log_unrecoverable_error;

pub(crate) fn get_package_from_file(file_path: &Path) -> String {
    let mut clone_file_path = file_path.clone().to_path_buf();
    clone_file_path.pop();
    get_package_from_dir(&clone_file_path)
}

pub(crate) fn get_package_from_dir(dir_path: &Path) -> String {
    if !dir_path.exists() || !dir_path.is_dir() {
        log_unrecoverable_error(
            format!(
                "Java package scanner called with an invalid folder parameter:\n\t\"{}\"",
                path_helper::to_absolute_path_str(dir_path)
            )
            .as_str(),
        );
    }

    get_package_from_dir_no_check(dir_path)
}

pub(crate) fn get_package_from_dir_no_check(dir_path: &Path) -> String {
    for ancestor in dir_path.ancestors() {
        if ancestor.ends_with("java") {
            if let Some(second_ancestor) = ancestor.parent() {
                if second_ancestor.ends_with("main") {
                    if let Some(third_ancestor) = second_ancestor.parent() {
                        if third_ancestor.ends_with("src") {
                            let bytes = ancestor.to_string_lossy().as_bytes().len();
                            let mut package_route = dir_path.to_string_lossy().to_string()[bytes..]
                                .to_owned()
                                .replace(['/', '\\'], ".");
                            package_route.remove(0); // Remove first "."
                            return package_route;
                        }
                    }
                }
            }
        }
    }

    "".to_string()
}

pub fn get_src_main_java_dir(path: &Path) -> Option<PathBuf> {
    if let Some(src_dir) = directory_browser::get_dir(path, "src") {
        if let Some(main_dir) = directory_browser::get_dir(&src_dir, "main") {
            if let Some(java_dir) = directory_browser::get_dir(&main_dir, "java") {
                return Some(java_dir);
            }
        }
    }

    None
}

pub fn check_base_java_project(path: &Path) {
    let mut files = Vec::new();
    files.push("build.gradle");
    files.push("pom.xml");

    if file_browser::get_first_file_if_exists(path, files).is_none() {
        panic!("Invalid java project root path found: {:?}", path);
    }
}

pub fn should_scan_dir(dir_path: &Path) -> bool {
    if dir_path.ends_with("java") {
        return !dir_path.ends_with("src/test/java");
    } else if dir_path.ends_with("target") || dir_path.ends_with(".mvn") {
        return false;
    }

    true
}
