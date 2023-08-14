use rusqlite::{Connection, Result};

use crate::chat::{create_chat, Chat, CREATE_CHAT_TABLE_QUERY};
use crate::helpers::generate_timestamp_id;
use crate::message::{
    create_message, create_message_type, Message, MessageType, CREATE_ASSET_TABLE_QUERY,
    CREATE_ASSET_TYPE_TABLE_QUERY, CREATE_MESSAGE_TABLE_QUERY, CREATE_MESSAGE_TYPE_TABLE_QUERY,
};
use crate::user::{create_user, User, CREATE_USER_TABLE_QUERY};

pub fn setup(conn: &Connection) -> Result<()> {
    conn.execute(CREATE_USER_TABLE_QUERY, ())?;
    conn.execute(CREATE_CHAT_TABLE_QUERY, ())?;
    conn.execute(CREATE_MESSAGE_TABLE_QUERY, ())?;
    conn.execute(CREATE_MESSAGE_TYPE_TABLE_QUERY, ())?;
    conn.execute(CREATE_ASSET_TABLE_QUERY, ())?;
    conn.execute(CREATE_ASSET_TYPE_TABLE_QUERY, ())?;

    //let test_chat_id = generate_timestamp_id();
    //let test_chat = Chat {
    //    id: test_chat_id.clone(),
    //    chat_id: 575757,
    //    name: "chat test".into(),
    //    //created_at: "2023-08-11".into(),
    //};
    //let test_chat = Chat::new(575757, "chat test".into());
    let test_chat = Chat::builder()
        .id(575757)
        .chat_id(8919164)
        .name("Tanque".into())
        .build();
    let test_chat_id = test_chat.id();
    println!("new test_chat_id is: {:?}", test_chat_id);
    create_chat(conn, &test_chat);

    let test_user_id = generate_timestamp_id();
    let test_user = User {
        id: test_user_id.clone(),
        user_id: 342432,
        name: "Linus Torvalds".into(),
        first_name: "Linus".into(),
        last_name: "Torvalds".into(),
        created_at: "2023-08-11".into(),
    };

    create_user(conn, &test_user);

    let test_message_type_id = generate_timestamp_id();
    let test_message_type = MessageType {
        id: test_message_type_id.clone(),
        name: "message".into(),
        created_at: "2023-08-11".into(),
    };
    create_message_type(conn, &test_message_type);

    println!("chat_id: {:?}", test_chat_id);
    println!("user_id: {:?}", test_user_id);
    println!("message_type_id: {:?}", test_message_type_id);

    let test_message_id = generate_timestamp_id();
    let test_message = Message {
        id: test_message_id.into(),
        message_id: generate_timestamp_id(),
        chat: test_chat_id.into(),
        author: test_user_id.into(),
        message_type: test_message_type_id.into(),
        text: Some("test".into()),
        date: "test".into(),
        edited: Some("test".into()),
        replying_to: Some(generate_timestamp_id()),
        duration_sec: Some(generate_timestamp_id()),
        created_at: "test".into(),
    };

    create_message(conn, &test_message);

    Ok(())
}
