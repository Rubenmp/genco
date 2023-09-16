use std::path::Path;

use crate::core::database::db_setup;
use crate::core::database::model::java_import_route::java_import_route_entity::JavaImportRouteEntity;
use crate::core::file_system::path_helper::try_to_absolute_path;

pub(crate) fn by_last_type_id(type_id: &str) -> Vec<JavaImportRouteEntity> {
    let conn = db_setup::get_db_connection();

    let mut stmt = conn
        .prepare(
            "SELECT id, base_package, route, last_type_id \
         FROM java_import_route \
         WHERE last_type_id = ?1",
        )
        .expect("Database statement preparation failed (\"by_last_type_id\")");

    stmt.query_map([type_id], |row| Ok(JavaImportRouteEntity::from_row(row)))
        .expect("Search JavaImportRoute by_last_type_id query failed")
        .filter_map(|row| row.ok())
        .collect()
}

pub(crate) fn by_base_package_and_route(
    base_package: &Path,
    import_route: &str,
) -> Vec<JavaImportRouteEntity> {
    let conn = db_setup::get_db_connection();

    let base_package_str = try_to_absolute_path(base_package);
    let mut stmt = conn
        .prepare(
            "SELECT id, base_package, route, last_type_id \
         FROM java_import_route \
         WHERE base_package = ?1 AND route = ?2",
        )
        .expect("Database statement preparation failed (\"by_base_package_and_route\")");

    stmt.query_map([base_package_str, import_route.to_owned()], |row| {
        Ok(JavaImportRouteEntity::from_row(row))
    })
    .expect("Search JavaImportRoute by_base_package_and_route query failed")
    .filter_map(|row| row.ok())
    .collect()
}

#[cfg(test)]
mod tests {
    use crate::core::database::model::java_import_route::db_java_import_route_search::by_last_type_id;

    #[test]
    fn search_test() {
        let result = by_last_type_id("fake");

        for person in result {
            println!("Found person {:?}", person);
        }
    }
}
