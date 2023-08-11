use std::fs::File;
use serde_json::{Value,};

use rusqlite::{Connection, Result};

mod globals;
mod helpers;
mod database;
mod chat;
mod message;
mod user;

use chat::{load_from_json_file, JSONChat};
use helpers::{generate_id, generate_timestamp_id, get_possible_message_fields};
use database::setup;

fn main() -> Result<()> {
    println!("Hello, world!");

    // let conn = Connection::open("persons.db")?;
    let conn = Connection::open(globals::DB_PATH)?;
    setup(&conn)?;

    let parse_fields_test = get_possible_message_fields(globals::TEST_JSON_PATH);
    let parsed_chat: JSONChat = load_from_json_file(globals::TEST_JSON_PATH);

    for message in &parsed_chat.messages {
        println!("### MESSAGE START ###");
        //println!("{:?}", message);
        if let Some(text) = &message.text {
            match text {
                Value::Array(String) => println!("text is an array"),
                //_ => println!("text is none"),
                _ => continue,
            }
        }
        println!("### MESSAGE END ###");
    }

    println!("{}", format!("This chat is with user: {}(id: {})\ntotal messages: {}", parsed_chat.name, parsed_chat.chat_id, parsed_chat.messages.len()));

    Ok(())
}
