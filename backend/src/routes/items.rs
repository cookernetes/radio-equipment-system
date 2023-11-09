use futures::stream::StreamExt;
use std::sync::Arc;
use actix_web::{patch, get, post, web, Responder, HttpResponse};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{self, doc};
use mongodb::Database;
use serde::{Serialize, Deserialize};
use crate::models::item_model::{Item, ItemStatus};

#[get("/items")]
pub async fn get_all_items(db: web::Data<Arc<Database>>) -> impl Responder {
    let collection = db.collection::<Item>("items");
    let items_cursor = collection.find(None, None).await.unwrap();
    let items: Vec<Result<_, _>> = items_cursor.collect().await;

    let unwrapped_items: _ = items.into_iter().flatten().collect::<Vec<_>>();

    HttpResponse::Ok().json(unwrapped_items)
}

#[derive(Serialize, Deserialize)]
struct CreateItemBody {
    name: String,
    status: Option<ItemStatus>,
    quantity: u8,
    image_uri: Option<String>,
    location_id: ObjectId, // TODO: Create routes to do with locations
}

#[post("/items/create")]
pub async fn create_item(db: web::Data<Arc<Database>>, body: web::Json<CreateItemBody>) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let collection = db.collection::<Item>("items");

    // TODO: in the future, can we/should we add more guards to items being created to avoid clones?
    if let Ok(Some(i)) = collection.find_one(doc!{ "name": body.name.clone().trim() }, None).await { return Ok(HttpResponse::Conflict().finish()) }

    let item_status = match &body.status {
        None => ItemStatus::Available,
        Some(status) => status.clone()
    };

    let new_item = Item {
        id: None,
        name: body.name.clone(),
        image_uri: body.image_uri.clone(),
        location_id: body.location_id.clone(),
        status: item_status,
        quantity: body.quantity.clone(),
        borrower_ids: vec![],
    };

    match collection.insert_one(new_item, None).await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()) // TODO: can we use a more descript error here?
    }
}

#[derive(Serialize, Deserialize)]
struct ChangeItemStatusRequestBody {
    item_id: ObjectId,
    new_status: ItemStatus,
}

#[patch("/change-item-status")]
pub async fn change_item_status(db: web::Data<Arc<Database>>, body: web::Json<ChangeItemStatusRequestBody>) -> impl Responder {
    let collection = db.collection::<Item>("items");

    let update_result = collection.update_one(
        doc! {
            "_id": body.item_id
        },
        doc! {
            "$set": {
                "status": bson::to_bson(&body.new_status).unwrap()
            }
        },
        None,
    ).await.unwrap();

    if update_result.matched_count == 0 {
        HttpResponse::NotFound().body("Error: No documents found to modify.")
    } else {
        HttpResponse::Ok().finish()
    }
}


// TODO: Below
#[get("/simple-stats")]
pub async fn get_simple_stats(db: web::Data<Arc<Database>>) -> impl Responder {
    return HttpResponse::InternalServerError();
    todo!();
}