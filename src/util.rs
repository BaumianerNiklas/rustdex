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

pub fn display_list<T>(list: Vec<T>, mapper: Box<dyn FnMut(&T) -> String>) -> String {
    list.iter().map(mapper).collect::<Vec<String>>().join(", ")
}

#[test]
fn normalize_text_works() {
    assert_eq!("Hello World", normalize_text("hElLo-WorLd"))
}
