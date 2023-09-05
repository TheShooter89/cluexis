use serde::{Deserialize, Serialize};

use crate::asset::Asset;
use crate::helpers::generate_timestamp_id;
use crate::message::JSONMessage;
use crate::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Chat {
    id: u64,
    pub chat_id: u64,
    pub name: String,
}

impl Chat {
    pub fn builder() -> ChatBuilder {
        ChatBuilder::default()
    }

    pub fn id(&self) -> u64 {
        self.id
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportedChat {
    pub chat: Chat,
    pub assets: Option<Vec<Asset>>,
    pub metadata: Option<ImportedChatMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportedChatMetadata {
    pub owner: (u64, String),
    pub total_messages: u64,
    pub total_assets: u64,
    pub users: Vec<(u64, String)>,
    pub oldest_message: u64,
    pub newest_message: u64,
    pub imported_on: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ChatBuilder {
    id: Option<u64>,
    chat_id: Option<u64>,
    name: Option<String>,
}

impl ChatBuilder {
    pub fn id(mut self, id: u64) -> Self {
        self.id = Some(id);
        self
    }

    pub fn chat_id(mut self, chat_id: u64) -> Self {
        self.chat_id = Some(chat_id);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn build(self) -> Chat {
        let id = self.id.unwrap_or_else(generate_timestamp_id);
        let chat_id = self.chat_id.expect("chat_id must be provided");
        let name = self.name.expect("name must be provided");

        Chat { id, chat_id, name }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JSONChat {
    pub name: String,
    #[serde(rename = "type")]
    pub chat_type: String,
    #[serde(rename = "id")]
    pub chat_id: i64,
    pub messages: Vec<JSONMessage>,
}

// ###########
// ## TESTS ##
// ###########

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_builder_with_id() {
        let chat = Chat::builder()
            .id(68618638)
            .chat_id(486682)
            .name("Tanque".to_string())
            .build();

        assert_eq!(chat.id(), 68618638);
        assert_eq!(chat.chat_id, 486682);
        assert_eq!(chat.name, "Tanque".to_string());
    }

    #[test]
    fn test_chat_builder_without_id() {
        let chat = Chat::builder()
            .chat_id(486682)
            .name("Tanque".to_string())
            .build();

        assert_eq!(chat.chat_id, 486682);
        assert_eq!(chat.name, "Tanque".to_string());
    }

    #[test]
    #[should_panic(expected = "chat_id must be provided")]
    fn test_missing_chat_id() {
        let chat = Chat::builder().name("prova".to_string()).build();
    }

    #[test]
    #[should_panic(expected = "name must be provided")]
    fn test_missing_name_id() {
        let chat = Chat::builder().chat_id(76899).build();
    }
}
