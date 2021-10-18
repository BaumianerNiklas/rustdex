use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Pokemon {
    pub id: u32,
    pub name: String,
    pub height: f32, // In decimetres
    pub weight: f32, // In hectograms
    pub stats: Vec<Stat>,
    pub abilities: Vec<Ability>,
    pub types: Vec<Type>,
}

#[derive(Deserialize, Debug)]
pub struct Stat {
    pub base_stat: u32,
    pub stat: StatReference,
}

#[derive(Deserialize, Debug)]
pub struct StatReference {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Ability {
    pub ability: AbilityReference,
    pub is_hidden: bool,
    pub slot: u8,
}

#[derive(Deserialize, Debug)]
pub struct AbilityReference {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Type {
    pub slot: u8,
    pub r#type: TypeReference,
}

#[derive(Deserialize, Debug)]
pub struct TypeReference {
    pub name: String,
    pub url: String,
}
