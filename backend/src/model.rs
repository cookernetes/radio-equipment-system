use mongodb::bson::doc;
use mongodb::Database;
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Deserialize, Serialize)]
pub enum UserRoleKind {
    Admin,
    User,
}

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
    pub image: Option<String>,
    pub quantity: u8,
    pub borrower_ids: Vec<String>,
    pub status: ItemStatus
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

pub async fn seed_db(db: &Database) {
    // Runs on first app start-up
    let coll = db.collection::<Item>("items");
    let doc_count = coll.count_documents(None, None).await.unwrap();

    if doc_count == 0 {
        println!("Seeding DB with initial items...");

        let seeding_doc = Item {
            id: None,
            name: "Example Item 1".to_string(),
            image: std::env::var("SEED_IMAGE_URL").ok(),
            quantity: 0,
            borrower_ids: vec![],
            status: ItemStatus::Available
        };

        let _ = coll.insert_one(seeding_doc, None).await.unwrap();
        println!("Successfully Seeded DB with initial items.");
    } else {
        println!("DB seeding not required.");
    }
}
