use rusqlite::{params, Connection, Error, Result};
use std::{fs::File, io::Read};

use crate::chat::{Chat, JSONChat, INSERT_CHAT_QUERY};

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
    conn.execute(
        INSERT_CHAT_QUERY,
        params![
            //
            chat.id(),
            chat.chat_id,
            chat.name,
        ],
    )
}
