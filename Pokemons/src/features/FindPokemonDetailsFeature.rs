

//use super::super::model::{Pokemon};

use crate::model::Pokemon;
use crate::jobs::GetPokemonDetailsFromWebApi::{getPokemonDetailsFromWebApi};
use crate::jobs::BuildPokemonDescription::{buildPokemonDescription};
use WebUtils::{transform_text_into_shakespeare_text};

pub async fn findPokemonDetailsFeature() {

    let pokemonDetails = getPokemonDetailsFromWebApi().await;

    let pokemonDescription = buildPokemonDescription(&pokemonDetails).await.expect("There should be a description");

    let shakespearePokemonDescription = transform_text_into_shakespeare_text(pokemonDescription.clone()).await.expect("There should be a shakesperean description");

    let pokemon = Pokemon {
        name: "charizard".to_owned(), 
        description: pokemonDescription,
        shakespearean_description:  shakespearePokemonDescription
    };

    panic!("{:#?}", pokemon) ;

    panic!("ok here");
}