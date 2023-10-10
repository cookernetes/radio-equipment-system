use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Deserialize, Serialize)]
pub enum UserRoleKind {
    Admin,
    User,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub full_name: String,
    pub username: String,
    pub pass_hash: String,
    pub role: UserRoleKind,
}