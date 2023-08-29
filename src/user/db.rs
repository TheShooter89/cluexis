use rusqlite::{params, Connection, Error, Result};
use std::{fs::File, io::Read};

use crate::user::{User, INSERT_USER_QUERY};

pub fn create_user(conn: &Connection, user: &User) -> Result<usize> {
    let first_name = match user.first_name.as_ref() {
        Some(name) => name.to_string(),
        None => "".to_string(),
    };

    conn.execute(
        INSERT_USER_QUERY,
        params![
            //
            user.id(),
            user.user_id,
            user.name,
            //user.first_name.unwrap_or(String::new()),
            user.first_name,
            //first_name,
            user.last_name,
        ],
    )
}
