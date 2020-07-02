extern crate hyper;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};


async fn pokemon_description(req: HttpRequest) -> impl Responder {
    format!("Hi {}!", &name)
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    panic!("aaa");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/pokemon/{name}", web::get().to(pokemon_description))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}





