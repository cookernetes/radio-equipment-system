use actix_web::{delete, HttpResponse, patch, post, web};
use std::sync::Arc;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::{bson, Database};
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

#[derive(Deserialize, Serialize)]
struct NewLocationStatusBody {
    location_id: ObjectId,
    new_status: LocationStatus
}

#[patch("/locations/edit-status")]
pub async fn edit_location_status(db: web::Data<Arc<Database>>, body: web::Json<NewLocationStatusBody>) -> HttpResponse {
    let collection = db.collection::<Location>("locations");

    let location_in_db = collection.find_one(doc! { "_id": body.location_id }, None).await.unwrap();

    if let Some(location) = location_in_db {
        let update_result = collection.update_one(
            doc! {
                "_id": location.id.unwrap()
            },
            doc! {
                "$set": {
                    "location_status": bson::to_bson(&body.new_status).unwrap()
                }
            },
            None
        ).await.unwrap();

        if update_result.matched_count == 0 {
            HttpResponse::NotFound().body("Error: No documents found to modify.")
        } else {
            HttpResponse::Ok().finish()
        }
    } else {
        HttpResponse::NotFound().body("Location with the specified ID is not in the DB")
    }
}

#[delete("/locations/{location_id}")]
pub async fn delete_location(db: web::Data<Arc<Database>>, path: web::Path<String>) -> HttpResponse {
    let (location_id) = path.into_inner();
    let collection = db.collection::<Location>("locations");

    return match collection.delete_one(doc! { "_id": location_id }, None).await {
        Ok(e) => {
            if e.deleted_count == 1 {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::BadRequest().body("Nothing was deleted - possibly malformed request with invalid location ID.")
            }
        },
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}