use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
    time::SystemTime,
};

use nanoid::nanoid;

use crate::chat::{Chat, JSONChat};

#[derive(Debug, Serialize, Deserialize)]
pub struct Parser {
    path: String,
}

impl Parser {
    pub fn new(path: &str) -> Self {
        Parser {
            path: path.to_string(),
        }
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn parse(&self) -> JSONChat {
        let mut raw_json = String::new();

        let mut source = File::open(self.path.as_str()).expect("error opening file");
        source
            .read_to_string(&mut raw_json)
            .expect("error while parsing JSON from the file");

        let chat: JSONChat = serde_json::from_str(&raw_json)
            .expect("error while deserializing JSON into Caht struct");

        println!("CHAT PARSED FROM JSON FILE");
        println!("chat is: {:?}", chat);

        return chat;
    }
}

// ###########
// ## TESTS ##
// ###########

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let parser = Parser::new("test");
        assert_eq!(parser.path(), "test".to_string());
    }
}
