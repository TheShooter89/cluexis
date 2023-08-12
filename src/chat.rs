use rusqlite::{params, Connection, Error, Result};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{fs::File, io::Read};

use crate::message::JSONMessage;

#[derive(Debug, Serialize, Deserialize)]
pub struct Chat {
    pub id: u64,
    pub chat_id: u64,
    pub name: String,
    pub created_at: String,
}

pub const CREATE_CHAT_TABLE_QUERY: &str = r#"
    CREATE TABLE IF NOT EXISTS chat (
        id INTEGER PRIMARY KEY,
        chat_id INTEGER NOT NULL,
        name TEXT NOT NULL,
        created_at TEXT NOT NULL
    )
"#;

pub const INSERT_CHAT_QUERY: &str = r#"
    INSERT INTO chat (
        id,
        chat_id,
        name,
        created_at
    ) VALUES (?1, ?2, ?3, ?4)
"#;

#[derive(Debug, Serialize, Deserialize)]
pub struct JSONChat {
    pub name: String,
    #[serde(rename = "type")]
    pub chat_type: String,
    #[serde(rename = "id")]
    pub chat_id: i64,
    pub messages: Vec<JSONMessage>,
}

pub fn load_from_json_file(path: &str) -> JSONChat {
    let mut json_content = String::new();

    let mut file = File::open(path).expect("error opening JSON file");
    file.read_to_string(&mut json_content)
        .expect("error serializing json");

    let chat: JSONChat =
        serde_json::from_str(&json_content).expect("error deserializing json content");

    println!("CHAT PARSED FROM JSON FILE");
    println!("chat is: {:?}", chat);

    return chat;
}

pub fn create_chat(conn: &Connection, chat: &Chat) -> Result<usize> {
    //
    //let new_id = generate_timestamp_id();
    conn.execute(
        INSERT_CHAT_QUERY,
        params![
            //
            chat.id,
            chat.chat_id,
            chat.name,
            chat.created_at,
        ],
    )
}
