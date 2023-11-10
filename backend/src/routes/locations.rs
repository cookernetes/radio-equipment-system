use actix_web::{HttpResponse, patch, post, web};
use std::sync::Arc;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use serde::{Deserialize, Serialize};
use crate::models::location_model::{Location, LocationStatus};

#[post("/locations/create")]
pub async fn create_location(db: web::Data<Arc<Database>>, body: web::Json<Location>) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let collection = db.collection::<Location>("locations");

    let location_exists = collection.find_one(doc! { "location_identifier": body.location_identifier.trim() }, None).await?;

    if let Some(_) = location_exists {
        return Ok(HttpResponse::Ok().body("Resource already exists"));
    }

    return match collection.insert_one(body.into_inner(), None).await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Ok(HttpResponse::InternalServerError().finish())
    }
}
