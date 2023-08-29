use serde::{Deserialize, Serialize};

use crate::helpers::generate_timestamp_id;
use crate::message::JSONMessage;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub user_id: u64,
    pub name: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

impl User {
    pub fn builder() -> UserBuilder {
        UserBuilder::default()
    }

    pub fn id(&self) -> u64 {
        self.id
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UserBuilder {
    id: Option<u64>,
    user_id: Option<u64>,
    name: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
}

impl UserBuilder {
    pub fn id(mut self, id: u64) -> Self {
        self.id = Some(id);
        self
    }

    pub fn user_id(mut self, user_id: u64) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn first_name(mut self, first_name: String) -> Self {
        self.first_name = Some(first_name);
        self
    }

    pub fn last_name(mut self, last_name: String) -> Self {
        self.last_name = Some(last_name);
        self
    }

    pub fn build(self) -> User {
        let id = self.id.unwrap_or_else(generate_timestamp_id);
        let user_id = self.user_id.expect("user_id must be provided");
        let name = self.name.expect("name must be provided");
        let first_name = self.first_name.or(None);
        let last_name = self.last_name.or(None);

        User {
            id,
            user_id,
            name,
            first_name,
            last_name,
        }
    }
}
