use std::fs;
use std::path::PathBuf;
use std::sync::Once;

use rusqlite::Connection;

static SYNC_OBJ: Once = Once::new();

/// WARN: there is a bottleneck and a bug here.
/// Current approach using "rusqlite::Connection" only handles individual connections
/// i.e. the connection is created from scratch everytime.
/// This is a problem in parallel scenarios (i.e. "cargo test"!!!) where several attempts
/// to get a database connection from different threads would lead to a panic.
///
/// Solution: migrate slowly to a pooled connection (and avoid repositories to use connection directly)
pub(crate) fn get_db_connection() -> Connection {
    SYNC_OBJ.call_once(|| {
        db_initial_migration();
    });

    get_db_connection_without_migration_attempt()
}

fn db_initial_migration() {
    let conn = get_db_connection_without_migration_attempt();
    let paths = fs::read_dir("./src/core/database/migrations")
        .expect("Database migrations path must exist");

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
    let base_db_folder = get_base_folder().join("database");
    let db_file = base_db_folder.join("test.db");

    fs::create_dir_all(base_db_folder)
        .expect("It was not possible to create the database directory");

    Connection::open(db_file).expect("connection open issue")
}

fn get_base_folder() -> PathBuf {
    std::env::current_dir()
        .expect("It was not possible to get base_dir for database setup")
        .to_path_buf()
}

#[cfg(test)]
mod tests {
    use crate::core::database::db_setup::get_db_connection;

    #[test]
    fn setup_test() {
        get_db_connection();
    }
}
