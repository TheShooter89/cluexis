pub const CREATE_USER_TABLE_QUERY: &str = r#"
    CREATE TABLE IF NOT EXISTS user (
        id INTEGER PRIMARY KEY,
        user_id INTEGER NOT NULL,
        name TEXT NOT NULL,
        first_name TEXT,
        last_name TEXT,
        created_at TIMESTAMP DEFAULT (datetime('now','localtime'))
    )
"#;

pub const INSERT_USER_QUERY: &str = r#"
    INSERT INTO user (
        id,
        user_id,
        name,
        first_name,
        last_name,
        created_at
    ) VALUES (?1, ?2, ?3, ?4, ?5, (datetime('now','localtime')))
"#;
