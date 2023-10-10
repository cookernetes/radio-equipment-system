use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
use crate::models::user_model::UserRoleType;

#[derive(Debug, Deserialize, Serialize)]
pub enum LocationType {
    Cupboard,
    Room,
    Shelf,
    Box,
    Drawer
}

#[derive(Debug, Deserialize, Serialize)]
pub enum LocationStatus {
    Available,
    Unavailable
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    // E.g. room number etc
    pub location_identifier: String,
    pub rbac_min_level: UserRoleType,
    pub location_type: LocationType,
    // The max capacity is optional, as not all locations have one/can have a max capacity estimated.
    pub max_capacity: Option<u16>,
    pub location_status: LocationStatus
}