use std::path::{Path, PathBuf};

use crate::core::file_system::directory_browsing::directory_browser;
use crate::core::file_system::file_browsing::file_browser;
use crate::core::file_system::path_helper;
use crate::core::parser::parser_node_trait::ParserNode;
use crate::java::parser::java_node::JavaNode;
use crate::java::parser::java_node_type::JavaNodeType;

/// This method will panic if the input path is not a valid dir within a java project (mvn/gradle)
pub(crate) fn get_base_package_unchecked(input_dir_path: &Path) -> PathBuf {
    if let Some(value) = get_base_package(input_dir_path) {
        return value;
    }

    panic!(
        "get_base_package_uncheck must exist, failed for path: {}",
        path_helper::try_to_absolute_path(input_dir_path)
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

pub(crate) fn get_package_route_from_file(file_path: &Path) -> Option<String> {
    get_package_route_opt_from_dir_no_check(&directory_browser::get_dir_of_file(file_path))
}

/// This method return the package option, like
/// - Some("org.test")
/// - None
/// assuming that input path is an existing dir
pub(crate) fn get_package_route_opt_from_dir_no_check(dir_path: &Path) -> Option<String> {
    for ancestor in dir_path.ancestors() {
        if ancestor.ends_with("java") {
            if let Some(second_ancestor) = ancestor.parent() {
                if second_ancestor.ends_with("main") {
                    if let Some(third_ancestor) = second_ancestor.parent() {
                        if third_ancestor.ends_with("src") {
                            return Some(get_package_route(dir_path, ancestor));
                        }
                    }
                }
            }
        }
    }

    None
}

pub(crate) fn should_scan_dir(dir_path: &Path) -> bool {
    if dir_path.ends_with("java") {
        return !dir_path.ends_with("src/test/java");
    } else if dir_path.ends_with("target") || dir_path.ends_with(".mvn") {
        return false;
    }

    true
}

pub(crate) fn find_package_start_end_bytes(
    input_java_file: &Path,
) -> Result<(usize, usize), String> {
    let package_search = JavaNode::new(input_java_file)?
        .depth_first_search_first_with_type(JavaNodeType::ScopedIdentifier);

    let java_node = package_search.ok_or(format!(
        "Package not found for java file:\n\"{}\"\n",
        path_helper::try_to_absolute_path(input_java_file)
    ))?;

    Ok((java_node.get_start_byte(), java_node.get_end_byte()))
}

fn contains_base_java_project_build_file(path: &Path) -> bool {
    let files = vec!["build.gradle", "pom.xml"];

    file_browser::get_first_file_from_dir_if_exists(path, files).is_some()
}

fn get_package_route(dir_path: &Path, ancestor: &Path) -> String {
    let bytes = ancestor.to_string_lossy().as_bytes().len();
    let mut package_route =
        dir_path.to_string_lossy().to_string()[bytes..].replace(['/', '\\'], ".");
    package_route.remove(0);
    package_route
}
