use rusqlite::{Connection, Result, params};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: u64,
    pub message_id: u64,
    pub chat: u64,
    pub author: u64,
    pub message_type: u64,
    pub text: Option<String>,
    pub date: String,
    pub edited: Option<String>,
    pub replying_to: Option<u64>,
    pub duration_sec: Option<u64>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageType {
    pub id: u64,
    pub name: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JSONMessage {
    pub id: i64,
    // below field on JSON is 'type'
    #[serde(rename = "type")]
    pub message_type: String,
    pub date: Option<String>,
    pub edited: Option<String>,
    pub reply_to_message_id: Option<i64>,
    pub forwarded_from: Option<String>,
    pub from: Option<String>,
    pub from_id: Option<String>,
    pub text: Option<Value>,
    pub title: Option<String>,
    pub media_type: Option<String>,
    pub file: Option<String>,
    pub duration_seconds: Option<i64>,
    pub photo: Option<String>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    // location_information is an object
    pub location_information: Option<Value>,
    // contact_information is an object
    pub contact_information: Option<Value>,
    pub thumbnail: Option<String>,
    pub via_bot: Option<String>,
    pub contact_vcard: Option<String>,
    pub sticker_emoji: Option<String>,
    pub action: Option<String>,
    // message_id referring to action field, usually for pinned messages
    pub message_id: Option<i64>,
    pub discard_reason: Option<String>,
    pub mime_type: Option<String>,
    pub performer: Option<String>,
    pub self_destruct_period_seconds: Option<i64>,
    pub live_location_period_seconds: Option<i64>,
    pub actor_id: Option<String>,
    pub actor: Option<String>,
}

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
        created_at TEXT NOT NULL,
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
    ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
"#;

pub const CREATE_MESSAGE_TYPE_TABLE_QUERY: &str = r#"
    CREATE TABLE IF NOT EXISTS message_type (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        created_at TEXT NOT NULL
    )
"#;

pub const INSERT_MESSAGE_TYPE_QUERY: &str = r#"
    INSERT INTO message_type (
        id,
        name,
        created_at
    ) VALUES (?1, ?2, ?3)
"#;

pub const CREATE_ASSET_TABLE_QUERY: &str = r#"
    CREATE TABLE IF NOT EXISTS asset (
        id INTEGER PRIMARY KEY,
        message INTEGER NOT NULL,
        type INTEGER NOT NULL,
        path TEXT NOT NULL,
        created_at TEXT NOT NULL,
        FOREIGN KEY (message) REFERENCES message (id),
        FOREIGN KEY (type) REFERENCES asset_type (id)
    )
"#;

pub const INSERT_ASSET_QUERY: &str = r#"
    INSERT INTO asset (
        id,
        message,
        type,
        path,
        created_at
    ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
"#;

pub const CREATE_ASSET_TYPE_TABLE_QUERY: &str = r#"
    CREATE TABLE IF NOT EXISTS asset_type (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        created_at TEXT NOT NULL
    )
"#;

pub const INSERT_ASSET_TYPE_QUERY: &str = r#"
    INSERT INTO asset_type (
        id,
        name,
        created_at
    ) VALUES (?1, ?2, ?3)
"#;

pub fn create_message(conn: &Connection, message: &Message) -> Result<()> {
    //
    //let new_id = generate_timestamp_id();
    //let new_text = match &message.text {
    //    Some(text) => text,
    //    None => None,
    //};
    match conn.execute(
        INSERT_MESSAGE_QUERY,
        params![
            //
            message.id,
            message.message_id,
            message.chat,
            message.author,
            message.message_type,
            message.text,
            message.date,
            message.edited,
            message.replying_to,
            message.duration_sec,
            message.created_at,
        ],
    ) {
            Ok(res) => println!("create_message function, response is: {:?}", res),
            Err(err) => println!("[ERROR] create_message function, error is: {:?}", err)
    }
    Ok(())
}

pub fn create_message_type(conn: &Connection, message_type: &MessageType) -> Result<()> {
    conn.execute(
        INSERT_MESSAGE_TYPE_QUERY,
        params![
            message_type.id,
            message_type.name,
            message_type.created_at
        ]
    )?;
    Ok(())
}

