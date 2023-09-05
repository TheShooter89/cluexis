use serde_json::Value;
use std::fs::File;

use rusqlite::{Connection, Result};

mod asset;
mod chat;
mod database;
mod globals;
mod helpers;
mod message;
mod parser;
mod user;

use chat::{create_chat, load_from_json_file, Chat, JSONChat};
use database::{load_mock_data, setup};
use helpers::{generate_id, generate_timestamp_id, get_possible_message_fields};
use parser::Parser;

fn main() -> Result<()> {
    println!("Hello, world!");

    // let conn = Connection::open("persons.db")?;
    let conn = Connection::open(globals::DB_PATH)?;
    setup(&conn)?;
    load_mock_data(&conn)?;

    let parse_fields_test = get_possible_message_fields(globals::TEST_JSON_PATH);
    //let parsed_chat: JSONChat = load_from_json_file(globals::TEST_JSON_PATH);

    let parser = Parser::new(globals::TEST_JSON_PATH);
    let parsed_chat: JSONChat = parser.parse();

    for message in &parsed_chat.messages {
        //println!("### MESSAGE START ###");
        //println!("{:?}", message);
        //if let Some(text) = &message.text {
        //    match text {
        //        Value::Array(String) => println!("text is an array"),
        //        //_ => println!("text is none"),
        //        _ => continue,
        //    }
        //}
        match &message.date {
            Some(_) => continue,
            None => println!(
                "\n---\ndate field absent\n[id]: {:?}\n[type]: {:?}\n---",
                message.id, message.message_type
            ),
        }
        match message.message_type.as_str() {
            "unsupported" => {
                //
                println!(
                    "unsupported message_type, message id: {:?}\n---",
                    message.id.to_string()
                )
            }
            _ => continue,
        }
        //println!("### MESSAGE END ###");
    }

    println!(
        "{}",
        format!(
            "This chat is with user: {}(id: {})\ntotal messages: {}",
            parsed_chat.name,
            parsed_chat.chat_id,
            parsed_chat.messages.len()
        )
    );

    let parsed_chat = load_from_json_file(globals::TEST_JSON_PATH);
    println!("parsed chat via Parser: {:?}", parsed_chat);

    for msg in parsed_chat.messages {
        println!("+----+----+----+----+----+----+----+----+----+----+");
        println!("message found, message id: {:?}", msg.id);
        println!("+----+----+----+----+----+----+----+----+----+----+");
    }

    //let andrea_chat = Chat::new(parsed_chat.chat_id as u64, parsed_chat.name.clone());
    //println!(
    //    "parsed chat fields still accessible, {:?}, {:?}",
    //    parsed_chat.chat_id, parsed_chat.name
    //);
    //println!("chat object is: {:?}", andrea_chat);

    //create_chat(&conn, &andrea_chat);

    Ok(())
}
