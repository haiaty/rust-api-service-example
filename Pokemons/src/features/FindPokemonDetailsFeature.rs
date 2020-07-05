use crate::model::Pokemon;
use crate::jobs::GetPokemonDetailsFromWebApi::{getPokemonDetailsFromWebApi};
use crate::jobs::BuildPokemonDescription::{buildPokemonDescription};
use WebUtils::{transform_text_into_shakespeare_text};


/**
 * Feature to find the pokemon details.
 */
pub async fn findPokemonDetailsFeature(pokemonName: &str) -> Result<Pokemon, anyhow::Error> {

    //================
    // Get the details
    //================
    // first of all we execute the 
    // step of getting the details from a remote web api
    let pokemonDetails = getPokemonDetailsFromWebApi(pokemonName).await?;

    //================
    // Build the description
    //================
    // then we execute the step of building the
    // description that would be used later 
    let pokemonDescription = buildPokemonDescription(&pokemonDetails).await?;

    //================
    // Translate the description
    //================
    // we translate the description into
    // a shakesperean text  
    let shakespearePokemonDescription = transform_text_into_shakespeare_text(pokemonDescription.clone()).await.expect("There should be a shakesperean description");

    //================
    // contruct the Model
    //================
    // after getting all the information needed
    // we construct the model. 
    // Note: the model should not be used in feature but only in jobs according 
    // to lucid principles but in this case it's ok to do it here 
    let pokemon = Pokemon {
        name: pokemonName.to_string(), 
        description:  shakespearePokemonDescription.to_owned()
    };
    //================
    // Return 
    //================
    // after we've finished all steps we return the result
    Ok(pokemon)

}


#[tokio::test]
async fn it_should_have_error_if_pokemon_does_not_exist() {

    
    // Setup
    let pokemonName = "aaa";

    // execute
    let bad_result = findPokemonDetailsFeature(pokemonName).await;

    // Assert
    assert!(bad_result.is_err() && !bad_result.is_ok());


}


#[tokio::test]
async fn it_should_give_a_result_with_a_pokemon() {

    
    // Setup
    let pokemonName = "pikachu";

    // execute
    let goodResult = findPokemonDetailsFeature(pokemonName).await;

    // Assert
    assert!(goodResult.is_ok());

    let pokemon =  goodResult.unwrap();

    assert_eq!(pokemonName, pokemon.name);


}

