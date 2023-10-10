use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Deserialize, Serialize)]
pub enum ItemStatus {
    InUse,
    Available,
    Unavailable
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub status: ItemStatus,
    pub quantity: u8,
    pub image_uri: Option<String>,
    pub borrower_ids: Vec<ObjectId>,
    pub location_id: ObjectId
}