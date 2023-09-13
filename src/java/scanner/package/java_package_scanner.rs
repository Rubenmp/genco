use std::path::{Path, PathBuf};

use crate::core::file_system::directory_browser::directory_browser;
use crate::core::file_system::file_browser::file_browser;
use crate::core::file_system::path_helper;
use crate::core::file_system::path_helper::try_to_absolute_path;
use crate::core::observability::logger::log_unrecoverable_error;

/// This method will panic if the input path is not a valid dir within a java project (mvn/gradle)
pub(crate) fn get_base_package_unchecked(input_dir_path: &Path) -> PathBuf {
    if let Some(value) = get_base_package(input_dir_path) {
        return value;
    }

    panic!(
        "get_base_package_uncheck must exist, failed for  path: {}",
        try_to_absolute_path(input_dir_path)
    )
}

pub(crate) fn get_base_package(input_dir_path: &Path) -> Option<PathBuf> {
    for ancestor in input_dir_path.ancestors() {
        if ancestor.ends_with("java") {
            if let Some(second_ancestor) = ancestor.parent() {
                if second_ancestor.ends_with("main") {
                    if let Some(third_ancestor) = second_ancestor.parent() {
                        if third_ancestor.ends_with("src") {
                            let mut base_java_project_buf = third_ancestor.to_path_buf();
                            base_java_project_buf.pop();
                            if !contains_base_java_project_build_file(&base_java_project_buf) {
                                return None;
                            }

                            return Some(base_java_project_buf);
                        }
                    }
                }
            }
        }
    }
    None
}

fn contains_base_java_project_build_file(path: &Path) -> bool {
    let files = vec!["build.gradle", "pom.xml"];

    file_browser::get_first_file_from_dir_if_exists(path, files).is_some()
}

pub(crate) fn get_package_from_dir(dir_path: &Path) -> String {
    if !dir_path.exists() || !dir_path.is_dir() {
        log_unrecoverable_error(
            format!(
                "Java package scanner called with an invalid folder parameter:\n\t\"{}\"",
                path_helper::try_to_absolute_path(dir_path)
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
                            return get_package_route(dir_path, ancestor);
                        }
                    }
                }
            }
        }
    }

    "".to_string()
}

fn get_package_route(dir_path: &Path, ancestor: &Path) -> String {
    let bytes = ancestor.to_string_lossy().as_bytes().len();
    let mut package_route = dir_path.to_string_lossy().to_string()[bytes..]
        .to_owned()
        .replace(['/', '\\'], ".");
    package_route.remove(0);
    package_route
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

pub fn should_scan_dir(dir_path: &Path) -> bool {
    if dir_path.ends_with("java") {
        return !dir_path.ends_with("src/test/java");
    } else if dir_path.ends_with("target") || dir_path.ends_with(".mvn") {
        return false;
    }

    true
}
