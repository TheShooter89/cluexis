use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::helpers::generate_timestamp_id;

#[derive(Debug, Serialize, Deserialize)]
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
}

impl Message {
    pub fn builder() -> MessageBuilder {
        MessageBuilder::default()
    }

    pub fn id(&self) -> u64 {
        self.id
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MessageBuilder {
    id: Option<u64>,
    message_id: Option<u64>,
    chat: Option<u64>,
    author: Option<u64>,
    message_type: Option<u64>,
    text: Option<String>,
    date: Option<String>,
    edited: Option<String>,
    replying_to: Option<u64>,
    duration_sec: Option<u64>,
}

impl MessageBuilder {
    pub fn id(mut self, id: u64) -> Self {
        self.id = Some(id);
        self
    }

    pub fn message_id(mut self, message_id: u64) -> Self {
        self.message_id = Some(message_id);
        self
    }

    pub fn chat(mut self, chat: u64) -> Self {
        self.chat = Some(chat);
        self
    }

    pub fn author(mut self, author: u64) -> Self {
        self.author = Some(author);
        self
    }

    pub fn message_type(mut self, message_type: u64) -> Self {
        self.message_type = Some(message_type);
        self
    }

    pub fn text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    pub fn date(mut self, date: String) -> Self {
        self.date = Some(date);
        self
    }

    pub fn edited(mut self, edited: String) -> Self {
        self.edited = Some(edited);
        self
    }

    pub fn replying_to(mut self, message_id: u64) -> Self {
        self.replying_to = Some(message_id);
        self
    }

    pub fn duration_sec(mut self, duration: u64) -> Self {
        self.duration_sec = Some(duration);
        self
    }

    pub fn build(self) -> Message {
        let id = self.id.unwrap_or_else(generate_timestamp_id);
        let message_id = self.message_id.expect("message_id must be provided");
        let chat = self.chat.expect("chat id must be provided");
        let author = self.author.expect("author id must be provided");
        let message_type = self.message_type.expect("message_type id must be provided");
        let text = self.text;
        let date = self.date.expect("date must be provided");
        let edited = self.edited;
        let replying_to = self.replying_to;
        let duration_sec = self.duration_sec;

        Message {
            id,
            message_id,
            chat,
            author,
            message_type,
            text,
            date,
            edited,
            replying_to,
            duration_sec,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageType {
    pub id: u64,
    pub name: String,
}

impl MessageType {
    pub fn builder() -> MessageTypeBuilder {
        MessageTypeBuilder::default()
    }

    pub fn id(&self) -> u64 {
        self.id
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MessageTypeBuilder {
    id: Option<u64>,
    name: Option<String>,
}

impl MessageTypeBuilder {
    pub fn id(mut self, id: u64) -> Self {
        self.id = Some(id);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn build(self) -> MessageType {
        let id = self.id.unwrap_or_else(generate_timestamp_id);
        let name = self.name.expect("name must be provided");

        MessageType { id, name }
    }
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

// ###########
// ## TESTS ##
// ###########

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_builder_with_id() {
        let message = Message::builder()
            .id(19890809)
            .message_id(486682)
            .chat(6189886)
            .author(789345)
            .message_type(5298693)
            .text("text test".to_string())
            .date("2023-08-09 08:09:08".to_string())
            .edited("2023-08-09 08:09:08".to_string())
            .replying_to(5298693)
            .duration_sec(5298693)
            .build();

        assert_eq!(message.id(), 19890809);
        assert_eq!(message.message_id, 486682);
        assert_eq!(message.chat, 6189886);
        assert_eq!(message.author, 789345);
        assert_eq!(message.message_type, 5298693);
        assert_eq!(message.text.unwrap(), "text test".to_string());
        assert_eq!(message.date, "2023-08-09 08:09:08".to_string());
        assert_eq!(message.edited.unwrap(), "2023-08-09 08:09:08".to_string());
        assert_eq!(message.replying_to.unwrap(), 5298693);
        assert_eq!(message.duration_sec.unwrap(), 5298693);
    }

    #[test]
    fn test_message_builder_without_id() {
        let message = Message::builder()
            .message_id(486682)
            .chat(6189886)
            .author(789345)
            .message_type(5298693)
            .text("text test".to_string())
            .date("2023-08-09 08:09:08".to_string())
            .edited("2023-08-09 08:09:08".to_string())
            .replying_to(5298693)
            .duration_sec(5298693)
            .build();

        assert_eq!(message.message_id, 486682);
        assert_eq!(message.chat, 6189886);
        assert_eq!(message.author, 789345);
        assert_eq!(message.message_type, 5298693);
        assert_eq!(message.text.unwrap(), "text test".to_string());
        assert_eq!(message.date, "2023-08-09 08:09:08".to_string());
        assert_eq!(message.edited.unwrap(), "2023-08-09 08:09:08".to_string());
        assert_eq!(message.replying_to.unwrap(), 5298693);
        assert_eq!(message.duration_sec.unwrap(), 5298693);
    }

    #[test]
    #[should_panic]
    fn test_missing_required_message_fields() {
        let message = Message::builder().date("prova".to_string()).build();
    }

    #[test]
    fn test_message_type_builder_with_id() {
        let message = MessageType::builder()
            .id(19890809)
            .name("message_type".to_string())
            .build();

        assert_eq!(message.id(), 19890809);
        assert_eq!(message.name, "message_type");
    }

    #[test]
    fn test_message_type_builder_without_id() {
        let message = MessageType::builder()
            .name("message_type".to_string())
            .build();

        assert_eq!(message.name, "message_type");
    }

    #[test]
    #[should_panic = "name must be provided"]
    fn test_missing_required_message_type_fields() {
        let message = MessageType::builder().build();
    }
}
