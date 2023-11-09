use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Deserialize, Serialize)]
pub enum UserRoleType {
    Admin,
    User,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserRawPhysicalToken {
    pub user_id: ObjectId,
    pub username: String,
    pub pass_hash: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub full_name: String,
    pub username: String,
    /**
        Their pass_hash is a 6 digit numerical passcode, which has to be used IN CONJUNCTION with their
        QR badge/ID card containing their token to be able to perform guarded actions.
    */
    pub pass_hash: String,
    pub physical_id_qr_token: String,
    pub role: UserRoleType,
}