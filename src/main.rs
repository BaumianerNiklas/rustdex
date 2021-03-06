mod model;
mod util;

use colored::Colorize;
use model::{evolution::PokemonEvolution, pokemon::Pokemon, species::PokemonSpecies};
use reqwest::blocking as reqwest;
use std::env;
use std::process;
use util::{
    display::{display_evolution_chain, display_list, display_pokedex_entry},
    functions::{get_stat_abbreviation, get_type_color, normalize_text, print_line},
};

fn main() {
    let pokemon_arg = env::args().skip(1).next();
    let client = reqwest::Client::new();

    let species = match pokemon_arg {
        None => {
            println!("An argument for a Pokémon was not provided.");
            process::exit(0);
        }
        Some(mut argument) => {
            argument = argument.to_lowercase();
            let url = format!("https://pokeapi.co/api/v2/pokemon-species/{}", argument);
            let response = client.get(url).send().unwrap();

            if response.status() == 404 {
                println!("The Pokémon {} wasn't found!", argument);
                process::exit(0);
            } else if !response.status().is_success() {
                println!(
                    "Something went wrong while querying PokéApi. (Status {}).",
                    response.status()
                );
                process::exit(1);
            }

            response.json::<PokemonSpecies>().unwrap()
        }
    };

    let pokemon_url = format!("https://pokeapi.co/api/v2/pokemon/{}", species.id);
    let pokemon_response = client.get(pokemon_url).send().unwrap();
    if !pokemon_response.status().is_success() {
        println!(
            "Something went wrong while querying PokéApi (Status {}).",
            pokemon_response.status()
        );
        process::exit(1);
    }
    let pokemon = pokemon_response.json::<Pokemon>().unwrap();

    let evolution_response = client.get(species.evolution_chain.url).send().unwrap();

    if !evolution_response.status().is_success() {
        println!(
            "Something went wrong while querying PokéApi (Status {}).",
            evolution_response.status()
        );
        process::exit(1)
    }
    let evolution = evolution_response.json::<PokemonEvolution>().unwrap();

    println!(
        "{} (#{})",
        normalize_text(&species.name).underline().bold(),
        &species.id
    );
    println!("{}", display_pokedex_entry(&species.flavor_text_entries));
    print_line(
        "Types",
        display_list(
            &pokemon.types,
            Box::new(|t| {
                let typeref = &t.r#type;
                let type_color = get_type_color(&typeref.name);

                normalize_text(&typeref.name)
                    .truecolor(type_color.0, type_color.1, type_color.2)
                    .to_string()
            }),
        ),
    );
    print_line("Weight", format!("{}kg", &pokemon.weight / 10.0));
    print_line("Height", format!("{}m", &pokemon.height / 10.0));
    print_line(
        "Stats",
        display_list(
            &pokemon.stats,
            Box::new(|s| format!("{}: {}", get_stat_abbreviation(&s.stat), &s.base_stat)),
        ),
    );
    print_line(
        "Abilities",
        display_list(
            &pokemon.abilities,
            Box::new(|a| {
                let normalized = normalize_text(&a.ability.name);
                if a.is_hidden {
                    normalized.underline().to_string()
                } else {
                    normalized
                }
            }),
        ),
    );
    let evolution_chain = display_evolution_chain(&evolution.chain);
    match evolution_chain {
        None => (),
        Some(evo_chain) => print_line("Evolution Chain", evo_chain),
    };
}
