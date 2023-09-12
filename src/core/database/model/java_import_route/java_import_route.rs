use std::ops::Add;
use std::path::{Path, PathBuf};

use rusqlite::Row;

use crate::core::file_system::file_browser::file_browser;
use crate::java::scanner::package::java_package_scanner;

#[derive(Debug)]
pub struct JavaImportRoute {
    id: i32,
    base_package: String,
    route: String,
    last_type_id: String,
}

impl JavaImportRoute {
    pub(crate) fn get_id(&self) -> i32 {
        self.id
    }
    pub(crate) fn get_base_package(&self) -> String {
        self.base_package.to_owned()
    }
    pub(crate) fn get_route(&self) -> String {
        self.route.to_owned()
    }
    pub(crate) fn get_last_type_id(&self) -> String {
        self.last_type_id.to_owned()
    }
}

#[derive(Debug)]
pub struct JavaImportRouteCreate {
    pub(crate) base_package: String,
    pub(crate) route: String,
    pub(crate) last_type_id: String,
}

impl JavaImportRouteCreate {
    pub(crate) fn new(file_path: &PathBuf) -> Self {
        let dir_path = Self::get_dir_path(file_path);
        let last_type_id = new_last_type_id(file_path);
        let (base_package, dir_route) =
            java_package_scanner::get_base_package_and_route_from_dir_no_check(&dir_path);

        JavaImportRouteCreate {
            base_package,
            route: dir_route.add(".").add(&last_type_id),
            last_type_id,
        }
    }

    fn get_dir_path(file_path: &PathBuf) -> PathBuf {
        let mut dir_path = file_path.to_owned();
        dir_path.pop();
        dir_path
    }
}

fn new_last_type_id(file_path: &Path) -> String {
    let file_name = file_path
        .iter()
        .last()
        .expect("Last type id must exist to transform path to JavaImportRoute")
        .to_str()
        .expect("Last type id must be transformed to string to convert it to JavaImportRoute")
        .to_string();
    file_browser::remove_java_extension(file_name)
}

impl JavaImportRoute {
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
