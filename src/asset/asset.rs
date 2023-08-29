use serde::{Deserialize, Serialize};

use crate::helpers::generate_timestamp_id;
use crate::message::JSONMessage;

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    id: u64,
    pub asset_type: u64,
    pub message: u64,
    pub path: String,
}

impl Asset {
    pub fn builder() -> AssetBuilder {
        AssetBuilder::default()
    }

    pub fn id(&self) -> u64 {
        self.id
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AssetBuilder {
    id: Option<u64>,
    asset_type: Option<u64>,
    message: Option<u64>,
    path: Option<String>,
}

impl AssetBuilder {
    pub fn id(mut self, id: u64) -> Self {
        self.id = Some(id);
        self
    }

    pub fn asset_type(mut self, asset_type_id: u64) -> Self {
        self.asset_type = Some(asset_type_id);
        self
    }

    pub fn message(mut self, message_id: u64) -> Self {
        self.message = Some(message_id);
        self
    }

    pub fn path(mut self, path: String) -> Self {
        self.path = Some(path);
        self
    }

    pub fn build(self) -> Asset {
        let id = self.id.unwrap_or_else(generate_timestamp_id);
        let asset_type = self.asset_type.expect("asset_type must be provided");
        let message = self.message.expect("message must be provided");
        let path = self.path.expect("path must be provided");

        Asset {
            id,
            asset_type,
            message,
            path,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetType {
    pub id: u64,
    pub name: String,
}

impl AssetType {
    pub fn builder() -> AssetTypeBuilder {
        AssetTypeBuilder::default()
    }

    pub fn id(&self) -> u64 {
        self.id
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AssetTypeBuilder {
    id: Option<u64>,
    name: Option<String>,
}

impl AssetTypeBuilder {
    pub fn id(mut self, id: u64) -> Self {
        self.id = Some(id);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn build(self) -> AssetType {
        let id = self.id.unwrap_or_else(generate_timestamp_id);
        let name = self.name.expect("name must be provided");

        AssetType { id, name }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JSONAsset {
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
    fn test_asset_builder_with_id() {
        let asset = Asset::builder()
            .id(68618638)
            .asset_type(486682)
            .message(8671769)
            .path("Tanque".to_string())
            .build();

        assert_eq!(asset.id(), 68618638);
        assert_eq!(asset.asset_type, 486682);
        assert_eq!(asset.message, 8671769);
        assert_eq!(asset.path, "Tanque".to_string());
    }

    #[test]
    fn test_asset_builder_without_id() {
        let asset = Asset::builder()
            .asset_type(486682)
            .message(8671769)
            .path("Tanque".to_string())
            .build();

        assert_eq!(asset.asset_type, 486682);
        assert_eq!(asset.message, 8671769);
        assert_eq!(asset.path, "Tanque".to_string());
    }

    #[test]
    #[should_panic(expected = "asset_id must be provided")]
    fn test_asset_builder_missing_asset_id() {
        let asset = Asset::builder().message(18756911).build();
    }

    #[test]
    #[should_panic(expected = "name must be provided")]
    fn test_asset_builder_missing_name_id() {
        let asset = Asset::builder().asset_type(76899).build();
    }

    #[test]
    fn test_asset_type_builder_with_id() {
        let asset = AssetType::builder()
            .id(19890809)
            .name("asset_type".to_string())
            .build();

        assert_eq!(asset.id(), 19890809);
        assert_eq!(asset.name, "asset_type");
    }

    #[test]
    fn test_asset_type_builder_without_id() {
        let asset = AssetType::builder().name("asset_type".to_string()).build();

        assert_eq!(asset.name, "asset_type");
    }

    #[test]
    #[should_panic = "name must be provided"]
    fn test_missing_required_asset_type_fields() {
        let asset = AssetType::builder().build();
    }
}
