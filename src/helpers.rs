use serde_json::Value;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
    time::SystemTime,
};

use nanoid::nanoid;

pub fn generate_id() -> String {
    let alphabet = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'd', 'g',
        'e', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
        'y', 'z',
    ];

    let id = nanoid!(8, &alphabet);
    return id;
}

pub fn generate_timestamp_id() -> u64 {
    let sys_time = SystemTime::now();
    let tv_sec = sys_time
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let tv_nsec = sys_time
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();

    let timestamp = tv_sec * 1_000_000_000 + u64::from(tv_nsec);

    return timestamp;
}

pub fn get_possible_message_fields(path: &str) -> HashMap<String, bool> {
    //
    let mut result_map: HashMap<String, bool> = HashMap::new();
    let mut possible_fields_map: HashMap<String, u32> = HashMap::new();
    let mut json_content = String::new();

    let mut file = File::open(path).expect("error opening json file");
    file.read_to_string(&mut json_content)
        .expect("error parsing json");

    let data: Value = serde_json::from_str(&json_content).expect("error serializing json");
    if let Some(messages) = data["messages"].as_array() {
        for message in messages {
            if let Value::Object(obj) = message {
                for key in obj.keys() {
                    match possible_fields_map.get(key) {
                        Some(count) => possible_fields_map.insert(key.clone(), count + 1),
                        None => possible_fields_map.insert(key.clone(), 1),
                    };
                }
            }
        }

        let total_messages = messages.len() as u32;
        println!("total messages in this chat: {total_messages}");

        for (key, value) in &possible_fields_map {
            match value == &total_messages {
                true => result_map.insert(key.clone(), true),
                false => result_map.insert(key.clone(), false),
            };
        }
        println!("result_map is: {:?}", result_map);
    }

    println!("possible_fields_map:");
    for (key, value) in &possible_fields_map {
        println!("{}: {}", key, value)
    }

    return result_map;
}
