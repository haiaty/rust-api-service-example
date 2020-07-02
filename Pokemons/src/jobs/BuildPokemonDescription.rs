
use WebUtils::{get_json_value_from_url, transform_text_into_shakespeare_text};
use serde_json::{Value};
use anyhow::Result;

pub async fn buildPokemonDescription(pokemonDetails : &Value) -> Result<String> {

    let mut pokemonDescrition = String::from("");

    let specieUrl = &pokemonDetails["species"]["url"];

    let specieDetails : Value = get_json_value_from_url(String::from(specieUrl.as_str().expect("there shoul be a string here"))).await?;

    let flavorEntries = specieDetails["flavor_text_entries"].as_array().unwrap();

    
    
   
    for e in flavorEntries.iter() {
        let obj = e.as_object().unwrap();

        let languageMap = obj["language"].as_object().unwrap();

        let language = languageMap["name"].as_str().unwrap();

        if language == "en"  {
            pokemonDescrition.push_str(obj["flavor_text"].as_str().unwrap());
        }

    }


    Ok(pokemonDescrition)
}