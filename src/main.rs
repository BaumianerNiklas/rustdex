mod model;
mod util;

use model::Pokemon;
use reqwest::blocking as reqwest;
use std::env;
use std::process;
use util::{display_list, normalize_text};

fn main() {
    let pokemon_arg = env::args().skip(1).next();
    match pokemon_arg {
        None => {
            println!("An argument for a Pokémon was not provided.");
            process::exit(0);
        }
        Some(mut argument) => {
            argument = argument.to_lowercase();
            let client = reqwest::Client::new();
            let url = format!("https://pokeapi.co/api/v2/pokemon/{}", argument);
            let response = client.get(url).send().unwrap();

            if response.status() == 404 {
                println!("The Pokémon {} wasn't found!", argument);
                process::exit(0);
            } else if !response.status().is_success() {
                println!(
                    "Something went wrong while querying PokéApi. (Status {})",
                    response.status()
                );
                process::exit(1);
            }

            let pokemon = response.json::<Pokemon>().unwrap();

            println!("{} (#{})", normalize_text(&pokemon.name), pokemon.id);
            println!(
                "Types: {}",
                display_list(pokemon.types, Box::new(|t| normalize_text(&t.r#type.name)))
            );
            println!(
                "Weight: {}kg\nHeight: {}m",
                pokemon.weight / 10.0,
                pokemon.height / 10.0
            );
            println!(
                "Stats: {}",
                display_list(
                    pokemon.stats,
                    Box::new(|s| format!("{}: {}", normalize_text(&s.stat.name), &s.base_stat))
                )
            );
            println!(
                "Abilities: {}",
                display_list(
                    pokemon.abilities,
                    Box::new(|a| normalize_text(&a.ability.name.clone()))
                )
            )
        }
    }
}
