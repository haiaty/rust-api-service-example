
//extern crate Pokemons;

use Pokemons::model::{Pokemon};
use Pokemons::features::FindPokemonDetailsFeature::{findPokemonDetailsFeature};
use anyhow::Result;

#[tokio::main]
async fn main()  -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    findPokemonDetailsFeature().await;
    Ok(())
}