
use WebUtils::fetch_json;
use serde_json::{Value};

/**
 * Get the details of a pokemon from 
 * a remote web service
 */
pub async fn getPokemonDetailsFromWebApi(pokemonName: &str) -> Result<Value, anyhow::Error> {
    
    //================
    // Build Uri
    //================
    // Here we build the uri that will be used to retrieve
    // pokemon information.
    //
    // NOTE: here I've hardcoded the base uri of the endpoint only for this test.
    // In production this may be taken from an .env file for example
    let baseUri = "https://pokeapi.co/api/v2/pokemon/";

    let mut fullUri  =  String::from("");
    fullUri.push_str(baseUri);
    fullUri.push_str(pokemonName);

    //==================
    // Execute call to remote service
    //==================
    let pokemonDetails : Value = fetch_json(fullUri).await?;

    //==================
    // return
    //==================
    Ok(pokemonDetails)

}