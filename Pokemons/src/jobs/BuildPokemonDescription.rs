
use WebUtils::{fetch_json};
use serde_json::{Value};
use anyhow::Result;

/**
 * Job to build a pokemon description given 
 * a Json Value of details
 */
pub async fn buildPokemonDescription(pokemonDetails : &Value) -> Result<String> {

    let mut pokemonDescrition = String::from("");

    //================
    // Get the url for enhance details
    //================
    // we first get the url that we will use to 
    // call another endpoint in order to have more details
    let specieUrl = &pokemonDetails["species"]["url"];


    //================
    // Get more details - specie details
    //================
    // we make the call to have more specie
    // details about the pokemon
    let specieDetails : Value = fetch_json(String::from(specieUrl.as_str().expect("there shoul be a string here"))).await?;


    //================
    // Build the description
    //================
    // the logic used here is a simple
    // concatenation of english texts
    let flavorEntries = specieDetails["flavor_text_entries"].as_array().unwrap();

    for e in flavorEntries.iter() {
        let obj = e.as_object().unwrap();

        let languageMap = obj["language"].as_object().unwrap();

        let language = languageMap["name"].as_str().unwrap();

        if language == "en"  {
            pokemonDescrition.push_str(obj["flavor_text"].as_str().unwrap());
        }

    }

    // here we remove some control chars
    // that may be find in the built pokemon
    // description
    let pokemonDescritionCleansed: String = pokemonDescrition.chars().filter(|c|  ! c.is_control()).collect();

    //================
    // Return
    //================
    Ok(pokemonDescritionCleansed)
}
