#![allow(non_snake_case)]
extern crate hyper;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use Pokemons::features::FindPokemonDetailsFeature::findPokemonDetailsFeature;
use Pokemons::model::{Pokemon};

async fn pokemon_description(req: HttpRequest) -> impl Responder {

    let pokemonName = req.match_info().get("name").unwrap_or("World");

    let result : Result<Pokemon, anyhow::Error> = findPokemonDetailsFeature(&pokemonName).await;

    if result.is_ok() {
        serde_json::to_string(&result.unwrap()).unwrap()
   
    } else {
        //In case of an error we let the user know
        // about the fact there was an error.
        // NOTE: we may want to keep the information abiut the error private and
        // let it be logged for us. Otherwise we may handle different cases 
        // and let the user know with more details
      r#"{"result": "there was an error"}"#.to_string()
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

     HttpServer::new(|| {
        App::new()
            .route("/pokemon/{name}", web::get().to(pokemon_description))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}