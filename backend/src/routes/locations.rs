use std::str::FromStr;
use actix_web::{delete, HttpResponse, patch, post, web};
use std::sync::Arc;
use mongodb::bson::doc;
use mongodb::bson::oid::{Error, ObjectId};
use mongodb::{bson, Database};
use serde::{Deserialize, Serialize};
use crate::models::location_model::{Location, LocationStatus};

#[post("/locations/create")]
pub async fn create_location(
    db: web::Data<Arc<Database>>,
    body: web::Json<Location>
) -> Result<HttpResponse, actix_web::Error> {
    let collection = db.collection::<Location>("locations");

    let location_exists = collection
        .find_one(doc! { "location_identifier": &body.location_identifier.trim() }, None)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Internal Server Error"))?;

    if location_exists.is_some() {
        return Err(actix_web::error::ErrorBadRequest("Resource already exists"));
    }

    collection
        .insert_one(body.into_inner(), None)
        .await
        .map(|_| HttpResponse::Ok().finish())
        .map_err(|_| actix_web::error::ErrorInternalServerError("Could not create location"))
}


#[derive(Deserialize, Serialize)]
struct NewLocationStatusBody {
    location_id: ObjectId,
    new_status: LocationStatus
}

#[patch("/locations/edit-status")]
pub async fn edit_location_status(
    db: web::Data<Arc<Database>>,
    body: web::Json<NewLocationStatusBody>
) -> Result<HttpResponse, actix_web::Error> {
    let collection = db.collection::<Location>("locations");

    let location_in_db = collection
        .find_one(doc! { "_id": &body.location_id }, None)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Error accessing the database"))?;

    if let Some(location) = location_in_db {
        let update_result = collection
            .update_one(
                doc! { "_id": location.id.expect("Location ID should be present") },
                doc! { "$set": { "location_status": bson::to_bson(&body.new_status).expect("Status should be convertible to BSON") } },
                None
            )
            .await
            .map_err(|_| actix_web::error::ErrorInternalServerError("Error updating the location"))?;

        if update_result.matched_count == 0 {
            Err(actix_web::error::ErrorNotFound("No documents found to modify"))
        } else {
            Ok(HttpResponse::Ok().finish())
        }
    } else {
        Err(actix_web::error::ErrorNotFound("Location with the specified ID is not in the DB"))
    }
}


#[delete("/locations/{location_id}")]
pub async fn delete_location(db: web::Data<Arc<Database>>, path: web::Path<String>) -> Result<HttpResponse, actix_web::Error> {
    let (location_id) = path.into_inner();
    let collection = db.collection::<Location>("locations");

    let location_oid = ObjectId::from_str(&location_id)
        .map_err(|_| actix_web::error::ErrorBadRequest("Malformed Location ID!"))?;

    let delete_result = collection.delete_one(doc! { "_id": location_oid }, None).await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Internal Server Error"))?;

    if delete_result.deleted_count == 1 {
        Ok(HttpResponse::Ok().finish())
    } else {
        Err(actix_web::error::ErrorBadRequest("Location not found/malformed ID."))
    }
}