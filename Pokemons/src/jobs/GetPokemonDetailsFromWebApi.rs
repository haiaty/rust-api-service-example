

// A Job is responsible for one element of execution in the application, 
// and play the role of a step in the accomplishment of a feature.

// no Job should dispatch another Job

//They can be ran by any Feature from any Service, 
// and it is the only way of communication between services and domains.

use WebUtils::get_json_value_from_url;
use serde_json::{Value};
//use WebUtils::{get_json_value_from_url};

pub async fn getPokemonDetailsFromWebApi() -> Value {
    
    let name = "charizard";

    let baseUri = "https://pokeapi.co/api/v2/pokemon/";

    let mut fullUri  =  String::from("");

    fullUri.push_str(baseUri);
    fullUri.push_str(name);

    let pokemonDetails : Value = get_json_value_from_url(fullUri).await.expect("Error getting from webapi");

    pokemonDetails

}