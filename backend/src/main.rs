use std::sync::Arc;
use crate::seed_db::{seed_db};
use actix_web::{App, HttpServer, Responder, middleware, web::{Data}, cookie::{Key, time}};
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_session::config::PersistentSession;
use mongodb::options::{ClientOptions, ResolverConfig};
use mongodb::Client;

mod utils;
mod seed_db;
mod routes;
mod models;

use routes::auth::{login, ping_test_route, ret_user_id_qr};
use routes::items::{change_item_status, get_all_items};
use crate::routes::items::{change_item_location, create_item};
use crate::routes::locations::{create_location, delete_location, edit_location_status};

const DB_NAME: &str = "inventory_tools";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenv::dotenv().ok();

    let mdb_uri = std::env::var("MONGODB_URI").expect("YOU MUST SET THE MONGODB_URI ENV VARIABLE!");
    let options = ClientOptions::parse_with_resolver_config(&mdb_uri, ResolverConfig::cloudflare())
        .await
        .unwrap();

    let mdb_client = Client::with_options(options).unwrap();
    let db = Arc::new(mdb_client.database(DB_NAME));
    seed_db(&db).await.expect("DB SEEDING FAILED! ");

    let secret_key: Key = Key::generate();

    println!("[!] Launching Server on http://127.0.0.1:3000/");
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(), secret_key.clone()
                )
                    // Short-lived session expires after 1 minute for security.
                    .session_lifecycle(
                        PersistentSession::default()
                            .session_ttl(time::Duration::minutes(1))
                    )
                    .build()
            )
            .app_data(Data::new(db.clone()))
            // .app_data(Data::new(sessions))
            .service(ping_test_route)
            .service(ret_user_id_qr)
            .service(get_all_items)
            .service(change_item_status)
            .service(login)
            .service(create_item)
            .service(change_item_location)
            .service(create_location)
            .service(edit_location_status)
            .service(delete_location)
    })
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}