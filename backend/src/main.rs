use std::sync::Arc;
use crate::seed_db::{seed_db};
use actix_web::{App, HttpServer, Responder, middleware, web::{Data}};
use mongodb::options::{ClientOptions, ResolverConfig};
use mongodb::Client;
use crate::routes::{change_item_status, get_all_items, ping_test_route};

mod seed_db;
mod routes;
mod models;

const DB_NAME: &str = "inventory_tools";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenv::dotenv().ok();

    let mdb_uri = std::env::var("MONGODB_URI").expect("YOU MUST SET THE MONGODB_URI ENV VARIABLE!");
    println!("{}", mdb_uri);

    let options = ClientOptions::parse_with_resolver_config(&mdb_uri, ResolverConfig::cloudflare())
        .await
        .unwrap();

    let mdb_client = Client::with_options(options).unwrap();
    let db = Arc::new(mdb_client.database(DB_NAME));

    seed_db(&db).await;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db.clone()))
            .wrap(middleware::Logger::default())
            .service(get_all_items)
            .service(ping_test_route)
            .service(change_item_status)
    })
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
