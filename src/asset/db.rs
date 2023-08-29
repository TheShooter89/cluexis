use rusqlite::{params, Connection, Error, Result};
use std::{fs::File, io::Read};

use crate::asset::{Asset, AssetType, JSONAsset, INSERT_ASSET_QUERY, INSERT_ASSET_TYPE_QUERY};

pub fn load_from_json_file(path: &str) -> JSONAsset {
    let mut json_content = String::new();

    let mut file = File::open(path).expect("error opening JSON file");
    file.read_to_string(&mut json_content)
        .expect("error serializing json");

    let asset: JSONAsset =
        serde_json::from_str(&json_content).expect("error deserializing json content");

    println!("CHAT PARSED FROM JSON FILE");
    println!("asset is: {:?}", asset);

    return asset;
}

pub fn create_asset(conn: &Connection, asset: &Asset) -> Result<usize> {
    conn.execute(
        INSERT_ASSET_QUERY,
        params![
            //
            asset.id(),
            asset.asset_type,
            asset.message,
            asset.path,
        ],
    )
}

pub fn create_asset_type(conn: &Connection, asset_type: &AssetType) -> Result<usize> {
    conn.execute(
        INSERT_ASSET_TYPE_QUERY,
        params![
            //
            asset_type.id(),
            asset_type.name,
        ],
    )
}
