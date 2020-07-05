
/**
 * Just a different "port" to 
 * use this module
 */
use Pokemons::features::FindPokemonDetailsFeature::{findPokemonDetailsFeature};
use anyhow::Result;

#[tokio::main]
async fn main()  -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let pokemonName = "pikachu";

    let pokemon = findPokemonDetailsFeature(&pokemonName).await?;

    let serialized = serde_json::to_string(&pokemon).unwrap();

    println!("Pokemon details is {:#?}", serialized);

    Ok(())
}