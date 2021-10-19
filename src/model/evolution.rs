use super::pokemon::SpeciesReference;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PokemonEvolution {
    pub chain: EvolutionChain,
}

#[derive(Deserialize, Debug)]
pub struct EvolutionChain {
    pub species: SpeciesReference,
    pub evolves_to: Vec<ChainItem>,
}

#[derive(Deserialize, Debug)]
pub struct ChainItem {
    pub species: SpeciesReference,
    pub evolves_to: Vec<ChainItem>,
}
