use rusqlite::{Connection, Result};

use crate::asset::{
    create_asset, create_asset_type, Asset, AssetType, CREATE_ASSET_TABLE_QUERY,
    CREATE_ASSET_TYPE_TABLE_QUERY,
};
use crate::chat::{create_chat, Chat, CREATE_CHAT_TABLE_QUERY};
use crate::helpers::generate_timestamp_id;
use crate::message::{
    create_message, create_message_type, Message, MessageType, CREATE_MESSAGE_TABLE_QUERY,
    CREATE_MESSAGE_TYPE_TABLE_QUERY,
};
use crate::user::{create_user, User, CREATE_USER_TABLE_QUERY};

pub fn setup(conn: &Connection) -> Result<()> {
    conn.execute(CREATE_USER_TABLE_QUERY, ())?;
    conn.execute(CREATE_CHAT_TABLE_QUERY, ())?;
    conn.execute(CREATE_MESSAGE_TABLE_QUERY, ())?;
    conn.execute(CREATE_MESSAGE_TYPE_TABLE_QUERY, ())?;
    conn.execute(CREATE_ASSET_TABLE_QUERY, ())?;
    conn.execute(CREATE_ASSET_TYPE_TABLE_QUERY, ())?;

    Ok(())
}

pub fn load_mock_data(conn: &Connection) -> Result<()> {
    //
    let mock_id_chat = generate_timestamp_id();
    let mock_id_user = generate_timestamp_id();
    let mock_id_message = generate_timestamp_id();

    let mock_chat = Chat::builder();
    let mock_user = User::builder();
    let mock_message = Message::builder();
    let mock_message_type = MessageType::builder();
    let mock_asset = Asset::builder();
    let mock_asset_type = AssetType::builder();

    //mock_chat.chat_id(mock_id_chat).name("Tanque".into());
    let chat = mock_chat
        .chat_id(mock_id_chat)
        .name("Tanque".into())
        .build();

    let user = mock_user
        .user_id(mock_id_user)
        .name("Tanque".into())
        .first_name("Francesco".to_string())
        .last_name("Paoletti".into())
        .build();

    let message_type = mock_message_type.name("message".into()).build();

    let message = mock_message
        .message_id(mock_id_message)
        .chat(chat.id().clone())
        .author(user.id().clone())
        .message_type(message_type.id().clone())
        .text("placeholder text".into())
        .date("2023-08-28".into())
        .build();

    let asset_type = mock_asset_type.name("image".into()).build();

    let asset = mock_asset
        .asset_type(asset_type.id().clone())
        .message(message.id().clone())
        .path("imaginary/path/to.file".into())
        .build();

    create_chat(conn, &chat)?;
    create_user(conn, &user)?;
    create_message_type(conn, &message_type)?;
    create_message(conn, &message)?;
    create_asset_type(conn, &asset_type)?;
    create_asset(conn, &asset)?;

    Ok(())
}
