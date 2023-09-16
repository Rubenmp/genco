use crate::core::database::db_setup;
use crate::core::database::model::java_import_route::java_import_route_entity::JavaImportRouteCreate;
use crate::core::observability::logger;

pub(crate) fn save(java_files: Vec<JavaImportRouteCreate>) -> Result<(), String> {
    for java_file in java_files {
        save_internal(java_file)?
    }

    Ok(())
}

fn save_internal(entity: JavaImportRouteCreate) -> Result<(), String> {
    let conn = db_setup::get_db_connection();

    match conn.execute(
        "INSERT INTO java_import_route (base_package, route, last_type_id) VALUES (?1, ?2, ?3)",
        (
            entity.base_package.to_owned(),
            entity.route.to_owned(),
            entity.last_type_id.to_owned(),
        ),
    ) {
        Ok(_n_saved_entities) => {}
        Err(err) => {
            let err_msg = format!("Error saving JavaImportRoute: {}", err);
            logger::log_warning(err_msg.as_str());
            return Err(err_msg);
        }
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::core::database::model::java_import_route::java_import_route_entity::JavaImportRouteCreate;
    use crate::core::database::model::java_import_route::{
        db_java_import_route_save, db_java_import_route_search,
    };

    #[test]
    fn save_test() {
        let last_type_id = "Autowired";
        let base_package = "/home/test_user/.m2/repository/org/springframework/spring-beans/5.3.27";
        let route = "org.springframework.beans.factory.annotation.Autowired";
        let entity_to_create = JavaImportRouteCreate {
            base_package: base_package.to_string(),
            route: route.to_string(),
            last_type_id: last_type_id.to_string(),
        };

        db_java_import_route_save::save_internal(entity_to_create).expect("Save should work");

        let result = db_java_import_route_search::by_last_type_id(last_type_id);

        assert_eq!(1, result.len());
        if let Some(result_item) = result.get(0) {
            assert_eq!(base_package, result_item.get_base_package());
            assert_eq!(route, result_item.get_route());
            assert_eq!(last_type_id, result_item.get_last_type_id());
        }
    }
}
