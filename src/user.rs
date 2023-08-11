use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub user_id: i64,
    pub name: String,
    pub first_name: String,
    pub last_name: String,
    pub created_at: String,
}

pub const CREATE_USER_TABLE_QUERY: &str = r#"
    CREATE TABLE IF NOT EXISTS user (
        id INTEGER PRIMARY KEY,
        user_id INTEGER NOT NULL,
        name TEXT NOT NULL,
        first_name TEXT NOT NULL,
        last_name TEXT NOT NULL,
        created_at TEXT NOT NULL
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
    ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
"#;

pub fn create_user(conn: &Connection, user: &User) -> Result<()> {
    //
    //let new_id = generate_timestamp_id();
    conn.execute(
        INSERT_USER_QUERY,
        params![
            //
            user.id,
            user.user_id,
            user.name,
            user.first_name,
            user.last_name,
            user.created_at,
        ],
    )?;
    Ok(())
}
