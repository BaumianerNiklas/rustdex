use crate::model::{evolution::EvolutionChain, species::FlavorTextEntry};
use crate::util::functions::normalize_text;
use colored::Colorize;

pub fn display_list<T>(list: &Vec<T>, mapper: Box<dyn FnMut(&T) -> String>) -> String {
    list.iter().map(mapper).collect::<Vec<String>>().join(", ")
}

pub fn display_pokedex_entry(entries: &Vec<FlavorTextEntry>) -> String {
    let entry = &entries
        .iter()
        .filter(|f| f.language.name == String::from("en"))
        .collect::<Vec<&FlavorTextEntry>>()[0]
        .flavor_text;

    entry
        .replace("\n", " ")
        .replace("\u{000c}", " ")
        .italic()
        .to_string()
}

pub fn display_evolution_chain(evo_chain: &EvolutionChain) -> Option<String> {
    let first_evo = &evo_chain.evolves_to;
    let evolves_to = match first_evo.len() {
        0 => return None,
        _ => first_evo,
    };
    let mut result = format!(
        "{} -> {}",
        normalize_text(&evo_chain.species.name),
        normalize_text(&evolves_to[0].species.name)
    );

    let second_evo = &evolves_to[0].evolves_to;
    match second_evo.len() {
        0 => Some(result),
        _ => {
            result += &format!(" -> {}", normalize_text(&second_evo[0].species.name));
            Some(result)
        }
    }
}
