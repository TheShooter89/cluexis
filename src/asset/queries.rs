pub const CREATE_ASSET_TABLE_QUERY: &str = r#"
    CREATE TABLE IF NOT EXISTS asset (
        id INTEGER PRIMARY KEY,
        type INTEGER NOT NULL,
        message INTEGER NOT NULL,
        path TEXT NOT NULL,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (type) REFERENCES asset_type (id),
        FOREIGN KEY (message) REFERENCES message (id)
    )
"#;

pub const INSERT_ASSET_QUERY: &str = r#"
    INSERT INTO asset (
        id,
        type,
        message,
        path,
        created_at
    ) VALUES (?1, ?2, ?3, ?4, CURRENT_TIMESTAMP)
"#;

pub const CREATE_ASSET_TYPE_TABLE_QUERY: &str = r#"
    CREATE TABLE IF NOT EXISTS asset_type (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )
"#;

pub const INSERT_ASSET_TYPE_QUERY: &str = r#"
    INSERT INTO asset_type (
        id,
        name,
        created_at
    ) VALUES (?1, ?2, CURRENT_TIMESTAMP)
"#;
