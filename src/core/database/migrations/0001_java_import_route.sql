CREATE TABLE IF NOT EXISTS java_import_route (
    id    INTEGER PRIMARY KEY AUTOINCREMENT,
    base_package  TEXT NOT NULL,
    route  TEXT NOT NULL,
    last_type_id  TEXT NOT NULL,
    UNIQUE(base_package, route) ON CONFLICT IGNORE
);
