use std::{env, str::FromStr, error::Error};
use actix_web::{get, Responder, HttpResponse, web, patch, post, HttpResponseBuilder};
use futures::stream::{StreamExt, TryStreamExt};
use std::sync::Arc;
use actix_session::Session;
use mongodb::bson::doc;
use mongodb::bson::oid::{ObjectId};
use mongodb::{bson, Database};
use serde::{Deserialize, Serialize};

use crate::models::item_model::{Item, ItemStatus};
use crate::models::user_model::{User, UserRawPhysicalToken};
use crate::utils::serialize_physical_token;

#[get("/ping")]
pub async fn ping_test_route() -> impl Responder {
    HttpResponse::ImATeapot().body("Pong!")
}

// TODO: Route guard.
#[get("/user-qr/{user_id}")]
pub async fn ret_user_id_qr(db: web::Data<Arc<Database>>, path: web::Path<String>) -> impl Responder {
    let user_id = path.into_inner();
    let user_id: ObjectId = match ObjectId::from_str(&user_id) {
        Ok(object_id) => object_id,
        Err(_) => return HttpResponse::BadRequest().finish()
    };

    let collection = db.collection::<User>("users");

    match collection.find_one(doc! {"_id": user_id}, None).await {
        Ok(None) => HttpResponse::NotFound().finish(),
        Ok(user) => {
            HttpResponse::Ok().body(user.unwrap().physical_id_qr_token)
        },
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[derive(Serialize, Deserialize)]
struct LoginRouteBody {
    /**
    User's 6 digit passcode.
    */
    pass: String,
    /**
    User's ID BRANCA token string
    */
    id_token: String,
    /**
    User's ID
    */
    user_id: ObjectId
}

#[post("/login")]
pub async fn login(session: Session, db: web::Data<Arc<Database>>, body: web::Json<LoginRouteBody>) -> Result<HttpResponse, Box<dyn Error>> {
    let collection = db.collection::<User>("users");
    // Get user pass hash from DB via user ID
    let user_req: _ = collection.find_one(doc! { "_id": body.user_id.clone() }, None).await?;

    let user: User = match user_req {
        None => return Ok(HttpResponse::NotFound().finish()),
        Some(user) => user
    };

    let parsed_token: UserRawPhysicalToken = match branca::decode(user.physical_id_qr_token.as_str(), env::var("BRANCA_KEY").unwrap().as_ref(), 0) {
        Ok(token) => serde_json::from_str(&String::from_utf8(token).unwrap()).unwrap(),
        Err(_) => return Ok(HttpResponse::Unauthorized().finish())
    };
    let UserRawPhysicalToken {username, pass_hash, user_id} = parsed_token;

    // ? Password Check
    let password_validity: bool = match bcrypt::verify(body.pass.clone().into_bytes(), &user.pass_hash) {
        Ok(value) => value,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish())
    };

    // Checks & Session Insertion
    if !password_validity || pass_hash != serialize_physical_token(user.physical_id_qr_token).unwrap().pass_hash || username != user.username {
        Ok(HttpResponse::Unauthorized().finish())
    } else {
        if let Some(_) = session.get::<ObjectId>("user_id")? {
            session.renew();
            Ok(HttpResponse::Ok().finish())
        } else {
            let _ = session.insert("user_id", user_id)?;

            Ok(HttpResponse::Ok().finish())
        }
    }
}

#[get("/items")]
pub async fn get_all_items(db: web::Data<Arc<Database>>) -> impl Responder {
    let collection = db.collection::<Item>("items");
    let items_cursor = collection.find(None, None).await.unwrap();
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