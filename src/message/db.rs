use rusqlite::{params, Connection, Error, Result};

use crate::message::{Message, MessageType, INSERT_MESSAGE_QUERY, INSERT_MESSAGE_TYPE_QUERY};

pub fn create_message(conn: &Connection, message: &Message) -> Result<usize> {
    conn.execute(
        INSERT_MESSAGE_QUERY,
        params![
            //
            message.id(),
            message.message_id,
            message.chat,
            message.author,
            message.message_type,
            message.text,
            message.date,
            message.edited,
            message.replying_to,
            message.duration_sec,
        ],
    )
}

pub fn create_message_type(conn: &Connection, message: &MessageType) -> Result<usize> {
    conn.execute(
        INSERT_MESSAGE_TYPE_QUERY,
        params![
            //
            message.id(),
            message.name,
        ],
    )
}
