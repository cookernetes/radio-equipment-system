use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
use mongodb::{Collection, Database};
use mongodb::error::Error;
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use url::Url;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ItemStatus {
    InUse,
    Available,
    Unavailable
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    #[serde(skip_serializing)]
    collection_string: &'static str,
    #[serde(skip_serializing)]
    collection: Collection<Self>,

    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub status: ItemStatus,
    pub quantity: u8,
    pub image_uri: Option<Url>,
    pub borrower_ids: Vec<ObjectId>,
    pub location_id: ObjectId
}

type MongoUpdateResult = mongodb::error::Result<UpdateResult>;

impl Item {
    async fn create(&self) -> mongodb::error::Result<InsertOneResult> {
        self.collection.insert_one(Self, None).await
    }

    async fn delete(&self) -> mongodb::error::Result<DeleteResult> {
        self.collection.delete_one(doc! { "_id": self.id }, None).await
    }

    async fn get_via_id(&self, oid: ObjectId) -> mongodb::error::Result<Option<Self>> {
        self.collection.find_one(doc! { "_id": oid }, None).await
    }

    async fn change_status(&mut self, status: ItemStatus) {
        self.status = status;
        self.collection.update_one(
            doc! {
                "_id": self.id
            },
            doc! {
                "$set": { "status": status }
            },
            None
        )
    }

    async fn change_location(&mut self, db: &Database, new_location_id: ObjectId) -> Option<MongoUpdateResult> {
        if let Ok(None) = db.collection("locations").find_one(doc! { "_id": new_location_id }, None).await {
            return None
        }

        self.location_id = new_location_id;
        Some(
            self.collection.update_one(
                doc! {
                "_id": self.id
            },
                doc! {
                "$set": { "location_id": new_location_id }
            },
                None
            ).await
        )
    }

    async fn add_borrower(&mut self, db: &Database, borrower_id: ObjectId) -> Option<MongoUpdateResult> {
        if let Ok(None) = db.collection("users").find_one(doc! {}, None).await {
            return None
        }

        self.borrower_ids.push(borrower_id);
        Some(
            self.collection.update_one(
                doc! {
                "_id": self.id
            },
                doc! {
                "$set": { "borrower_ids": self.borrower_ids }
            },
                None
            ).await
        )
    }

    async fn change_quantity(&mut self, new_quantity: u8) -> MongoUpdateResult {
        self.quantity = new_quantity;
        self.collection.update_one(
            doc! {
                "_id": self.id
            },
            doc! {
                "$set": { "quantity": self.quantity }
            },
            None
        ).await
    }

    async fn change_image_uri(&mut self, image_uri: Url) -> MongoUpdateResult {
        self.image_uri = Some(image_uri);
        self.collection.update_one(
            doc! {
                "_id": self.id
            },
            doc! {
                "$set": { "image_uri": self.image_uri }
            },
            None
        ).await
    }
}