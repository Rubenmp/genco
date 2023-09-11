use std::path::Path;

use rusqlite::Row;

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
    pub(crate) fn new(path: &Path) -> Self {
        todo!()
    }
}

impl JavaImportRoute {
    pub(crate) fn from_row(row: &Row) -> Self {
        Self {
            id: row.get(0).expect("JavaImportRoute field \"id\" missing"),
            base_package: row.get(1).expect("JavaImportRoute field \"name\" missing"),
            route: row.get(2).expect("JavaImportRoute field \"data\" missing"),
            last_type_id: row.get(3).expect("JavaImportRoute field \"data\" missing"),
        }
    }
}
