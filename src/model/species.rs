use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PokemonSpecies {
    pub evolution_chain: EvolutionChainReference,
    pub flavor_text_entries: Vec<FlavorTextEntry>,
    pub generation: GenerationReference,
}

#[derive(Deserialize, Debug)]
pub struct EvolutionChainReference {
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct FlavorTextEntry {
    pub flavor_text: String,
    pub language: LanguageReference,
}

#[derive(Deserialize, Debug)]
pub struct LanguageReference {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct GenerationReference {
    pub name: String,
    pub url: String,
}
