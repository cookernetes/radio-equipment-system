use actix_web::{get, Responder, HttpResponse, web, patch};
use futures::stream::{StreamExt, TryStreamExt};
use std::sync::Arc;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::{bson, Database};
use serde::{Deserialize, Serialize};
use crate::models::item_model::{Item, ItemStatus};

#[get("/ping")]
async fn ping_test_route() -> impl Responder {
    HttpResponse::ImATeapot().body("Pong!")
}

#[get("/items")]
async fn get_all_items(db: web::Data<Arc<Database>>) -> impl Responder {
    let collection = db.collection::<Item>("items");
    let mut items_cursor = collection.find(None, None).await.unwrap();
    let items: Vec<Result<_, _>> = items_cursor.collect().await;

    let unwrapped_items: _ = items.into_iter().flatten().collect::<Vec<_>>();

    HttpResponse::Ok().json(unwrapped_items)
}

#[derive(Serialize, Deserialize)]
struct ChangeItemStatusRequestBody {
    item_id: ObjectId,
    new_status: ItemStatus,
}

#[patch("/change-item-status")]
async fn change_item_status(db: web::Data<Arc<Database>>, body: web::Json<ChangeItemStatusRequestBody>) -> impl Responder {
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
        HttpResponse::Ok().into()
    }
}

// TODO: Below
#[get("/simple-stats")]
async fn get_simple_stats(db: web::Data<Arc<Database>>) -> impl Responder {
    return HttpResponse::InternalServerError();
    todo!();
}