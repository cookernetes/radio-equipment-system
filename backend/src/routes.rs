use actix_web::{get, Responder, HttpResponse, web, patch};
use futures::stream::{StreamExt, TryStreamExt};
use std::sync::Arc;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use serde::{Deserialize, Serialize};
use crate::model::{Item, ItemStatus};

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

/*#[patch("/change-item-status")]
async fn change_item_status(db: web::Data<Arc<Database>>) -> impl Responder {
    // Modify with mongodb in database, and if item is not found - send error message & HTTP error code accordingly.
    let collection = db.collection::<Item>("items");

    let filter = doc! {"name": };
    let update = doc! {};
    collection.update_one(doc! {}, doc! {})
}*/

#[get("/simple-stats")]
async fn get_simple_stats(db: web::Data<Arc<Database>>) -> impl Responder {
    return HttpResponse::InternalServerError();
    todo!();
}