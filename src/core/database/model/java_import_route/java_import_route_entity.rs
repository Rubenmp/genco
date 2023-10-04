use std::path::{Path, PathBuf};

use rusqlite::Row;

use crate::core::file_system::file_browsing::file_browser;
use crate::core::file_system::path_helper::try_to_absolute_path;
use crate::java::scanner::package::java_package_scanner;

#[derive(Debug)]
pub struct JavaImportRouteEntity {
    id: i32,
    base_package: String,
    route: String,
    last_type_id: String,
}

impl JavaImportRouteEntity {
    pub(crate) fn get_id(&self) -> i32 {
        self.id
    }
    pub(crate) fn get_base_package(&self) -> String {
        self.base_package.clone()
    }
    pub(crate) fn get_route(&self) -> String {
        self.route.clone()
    }
    pub(crate) fn get_last_type_id(&self) -> String {
        self.last_type_id.clone()
    }

    pub(crate) fn to_file_path(&self) -> PathBuf {
        let route_dot_replaces = self.get_route().replace('.', "/");
        let result_absolute_path = format!(
            "{}{}{}.java",
            self.get_base_package(),
            "/src/main/java/",
            route_dot_replaces
        );
        Path::new(&result_absolute_path).to_path_buf()
    }
}

#[derive(Debug)]
pub struct JavaImportRouteCreate {
    pub(crate) base_package: String,
    pub(crate) route: String,
    pub(crate) last_type_id: String,
}

impl JavaImportRouteCreate {
    /// Preconditions:
    /// - All the input paths are in the same dir which is within a java project
    /// - All files have a ".java" extension
    pub(crate) fn from(java_files_in_same_dir_unchecked: Vec<PathBuf>) -> Vec<Self> {
        if java_files_in_same_dir_unchecked.is_empty() {
            return vec![];
        }
        let base_package_path = java_package_scanner::get_base_package_unchecked(
            java_files_in_same_dir_unchecked
                .get(0)
                .expect("JavaImportRoute::from failed"),
        );

        let base_package_path_str = try_to_absolute_path(&base_package_path);
        let mut result = Vec::with_capacity(java_files_in_same_dir_unchecked.len());
        for file in java_files_in_same_dir_unchecked {
            let java_file_name = get_last_item_str_unchecked(&file);
            let file_path_str = try_to_absolute_path(&file);
            if let Some(route) = get_import_route(&base_package_path_str, &file_path_str) {
                let import = Self {
                    base_package: base_package_path_str.to_string(),
                    route,
                    last_type_id: file_browser::remove_java_extension(java_file_name),
                };

                result.push(import);
            }
        }

        result
    }

    fn get_dir_path(file_path: &Path) -> PathBuf {
        let mut dir_path = file_path.to_path_buf();
        dir_path.pop();
        dir_path
    }
}

/// This method never return a None due to the preconditions applied by the caller to the parameters.
/// Input examples:
/// - base_package_path: "/home/<user>/genco/src/java/dto/test/java_class"
/// - file_path: "/home/<user>/genco/src/java/dto/test/java_class/src/main/java/org/test/JavaClassFrom.java"
///
/// Expected result: "org.test.JavaClassFrom"
fn get_import_route(base_package_path: &str, file_path: &str) -> Option<String> {
    let start = base_package_path.as_bytes().len() + "/src/main/java/".as_bytes().len();
    let end = file_path.as_bytes().len() - ".java".as_bytes().len();
    if start >= end {
        return None;
    }
    let result = file_path[start..end].to_string().replace('/', ".");
    Some(result)
}

fn get_last_item_str_unchecked(file_path: &Path) -> String {
    file_path
        .iter()
        .last()
        .expect("Last type id must exist to transform path to JavaImportRoute")
        .to_str()
        .expect("Last type id must be transformed to string to convert it to JavaImportRoute")
        .to_string()
}

impl JavaImportRouteEntity {
    pub(crate) fn from_row(row: &Row) -> Self {
        Self {
            id: row.get(0).expect("JavaImportRoute field \"id\" missing"),
            base_package: row
                .get(1)
                .expect("JavaImportRoute field \"base_package\" missing"),
            route: row.get(2).expect("JavaImportRoute field \"route\" missing"),
            last_type_id: row
                .get(3)
                .expect("JavaImportRoute field \"last_type_id\" missing"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::database::model::java_import_route::java_import_route_entity;

    #[test]
    fn get_import_route_test() {
        let base_package_path = "/home/<user>/genco/src/java/dto/test/java_class";
        let file_path = "/home/<user>/genco/src/java/dto/test/java_class/src/main/java/org/test/JavaClassFrom.java";

        let result = java_import_route_entity::get_import_route(base_package_path, file_path);

        assert_eq!(Some("org.test.JavaClassFrom".to_string()), result);
    }
}
