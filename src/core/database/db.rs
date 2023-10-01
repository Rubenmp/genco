use rusqlite::Error;

use crate::core::database::db_setup;
use crate::core::observability::logger;

pub(crate) fn execute_insert_3_param(
    query: &str,
    params: (&str, &str, &str),
) -> Result<usize, String> {
    let conn = db_setup::get_db_connection();
    match conn.execute(query, params) {
        Ok(n) => Ok(n),
        Err(err) => log_execute_error(err),
    }
}

fn log_execute_error(err: Error) -> Result<usize, String> {
    let err_msg = format!("Error running execute query: {}", err);
    logger::log_warning(err_msg.as_str());
    Err(err_msg)
}
