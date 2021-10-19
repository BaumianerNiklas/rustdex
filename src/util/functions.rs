use crate::model::pokemon::StatReference;
use colored::Colorize;

pub fn print_line(name: &str, data: String) {
    println!("{}: {}", name.bold(), data)
}

pub fn normalize_text(text: &str) -> String {
    let lowercase_text = text.to_lowercase();
    let split = lowercase_text.split('-');
    let mut words: Vec<String> = vec![];
    split.for_each(|word| {
        let first_letter = word.chars().next();
        match first_letter {
            None => words.push(String::from("")),
            Some(letter) => {
                // This is cursed and horrible
                let normalized_word = format!(
                    "{}{}",
                    letter.to_uppercase(),
                    word.chars().collect::<Vec<char>>()[1..]
                        .iter()
                        .collect::<String>()
                );
                words.push(normalized_word);
            }
        }
    });
    words.join(" ")
}

pub fn get_type_color(type_name: &str) -> (u8, u8, u8) {
    match type_name {
        "normal" => (10, 170, 169),
        "fire" => (255, 68, 34),
        "water" => (51, 153, 255),
        "grass" => (119, 204, 85),
        "electric" => (244, 195, 48),
        "ice" => (102, 204, 255),
        "fighting" => (187, 85, 68),
        "poison" => (170, 85, 153),
        "ground" => (214, 181, 82),
        "flying" => (136, 153, 255),
        "psychic" => (255, 85, 153),
        "bug" => (170, 187, 34),
        "rock" => (187, 170, 102),
        "ghost" => (102, 102, 187),
        "dragon" => (119, 102, 238),
        "dark" => (119, 85, 68),
        "steel" => (170, 170, 187),
        "fairy" => (238, 153, 238),
        _ => (255, 255, 255),
    }
}

pub fn get_stat_abbreviation(stat: &StatReference) -> String {
    match stat.name.as_str() {
        "hp" => "HP",
        "attack" => "ATK",
        "defense" => "DEF",
        "special-attack" => "Sp.ATK",
        "special-defense" => "Sp.DEF",
        "speed" => "SPD",
        s => panic!("Encounterd unknown stat name: {}", &s),
    }
    .to_string()
}
