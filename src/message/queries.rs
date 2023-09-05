pub const CREATE_MESSAGE_TABLE_QUERY: &str = r#"
    CREATE TABLE IF NOT EXISTS message (
        id INTEGER PRIMARY KEY,
        message_id INTEGER NOT NULL,
        chat INTEGER NOT NULL,
        author INTEGER NOT NULL,
        type INTEGER NOT NULL,
        text TEXT,
        date TEXT NOT NULL,
        edited TEXT,
        replying_to INTEGER,
        duration_sec INTEGER,
        created_at TIMESTAMP DEFAULT (datetime('now','localtime')),
        FOREIGN KEY (chat) REFERENCES chat (id),
        FOREIGN KEY (author) REFERENCES user (id),
        FOREIGN KEY (type) REFERENCES message_type (id)
    )
"#;

pub const INSERT_MESSAGE_QUERY: &str = r#"
    INSERT INTO message (
        id,
        message_id,
        chat,
        author,
        type,
        text,
        date,
        edited,
        replying_to,
        duration_sec,
        created_at
    ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, (datetime('now','localtime')))
"#;

pub const CREATE_MESSAGE_TYPE_TABLE_QUERY: &str = r#"
    CREATE TABLE IF NOT EXISTS message_type (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        created_at TIMESTAMP DEFAULT (datetime('now','localtime'))
    )
"#;

pub const INSERT_MESSAGE_TYPE_QUERY: &str = r#"
    INSERT INTO message_type (
        id,
        name,
        created_at
    ) VALUES (?1, ?2, (datetime('now','localtime')))
"#;
