use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use mongodb::Client;
use mongodb::options::{ClientOptions, ResolverConfig};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenv::dotenv();
    let mdb_uri = std::env::var("MONGODB_URI").expect("YOU MUST SET THE MONGODB_URI ENV VARIABLE!");

    println!("{}", mdb_uri);

    let options =
        ClientOptions::parse_with_resolver_config(&mdb_uri, ResolverConfig::cloudflare())
            .await
            .unwrap();


    let mongodb = Client::with_options(options).unwrap();

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
    })
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
