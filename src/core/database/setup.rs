use std::fs;
use std::path::PathBuf;
use std::sync::Once;

use rusqlite::Connection;

static SYNC_OBJ: Once = Once::new();

pub(crate) fn get_db_connection() -> Connection {
    SYNC_OBJ.call_once(|| {
        db_initial_migration();
    });

    get_db_connection_without_migration_attempt()
}

fn db_initial_migration() {
    let conn = get_db_connection_without_migration_attempt();

    let paths = fs::read_dir("./src/core/database/migrations").unwrap();

    for path in paths {
        let file_path = path.unwrap().path();
        if file_path.exists() && file_path.is_file() {
            let filename = file_path
                .file_name()
                .expect("File name must exists")
                .to_str()
                .expect("File name str");
            if filename.ends_with(".sql") {
                let sql_query = fs::read_to_string(file_path).expect("Read to string");
                conn.execute(sql_query.as_str(), ())
                    .unwrap_or_else(|_| panic!("migration query: \"{}\"", sql_query));
            }
        }
    }
    conn.close().expect("Database connection must close");
}

fn get_db_connection_without_migration_attempt() -> Connection {
    Connection::open(get_database_file_path()).expect("connection open issue")
}

fn get_database_file_path() -> PathBuf {
    get_base_folder().join("database").join("test.db")
}

fn get_base_folder() -> PathBuf {
    std::env::current_dir().unwrap().to_path_buf()
}

#[cfg(test)]
mod tests {
    use crate::core::database::setup::get_db_connection;

    #[test]
    fn setup_test() {
        get_db_connection();
    }
}
