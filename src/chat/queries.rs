pub const CREATE_CHAT_TABLE_QUERY: &str = r#"
    CREATE TABLE IF NOT EXISTS chat (
        id INTEGER PRIMARY KEY,
        chat_id INTEGER NOT NULL,
        name TEXT NOT NULL,
        created_at TIMESTAMP DEFAULT (datetime('now', 'localtime'))
    )
"#;

pub const INSERT_CHAT_QUERY: &str = r#"
    INSERT INTO chat (
        id,
        chat_id,
        name,
        created_at
    ) VALUES (?1, ?2, ?3, (datetime('now', 'localtime')))
"#;
