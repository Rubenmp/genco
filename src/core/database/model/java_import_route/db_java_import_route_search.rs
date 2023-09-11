use crate::core::database::model::java_import_route::java_import_route::JavaImportRoute;
use crate::core::database::setup;

pub fn by_last_type_id(type_id: &str) -> Vec<JavaImportRoute> {
    let conn = setup::get_db_connection();

    let mut stmt = conn
        .prepare(
            "SELECT id, base_package, route, last_type_id \
         FROM java_import_route \
         WHERE last_type_id = ?1",
        )
        .expect("Database statement preparation failed (\"by_last_type_id\")");

    let mapped_rows_to_result = stmt
        .query_map([type_id], |row| Ok(JavaImportRoute::from_row(row)))
        .expect("Search JavaImportRoute query failed")
        .filter_map(|row| row.ok())
        .collect();

    mapped_rows_to_result
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
