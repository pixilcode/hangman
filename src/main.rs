use std::collections::HashSet;

mod game;

fn main() {
    let dictionary = construct_dictionary();

    let game = game::HangmanGame::new(dictionary);
}

fn construct_dictionary() -> HashSet<String> {
    let mut dictionary = HashSet::new();

    dictionary.insert("dazzling".to_string());
    dictionary.insert("mellow".to_string());
    dictionary.insert("breathe".to_string());
    dictionary.insert("awesome".to_string());
    dictionary.insert("tumble".to_string());
    dictionary.insert("offend".to_string());
    dictionary.insert("funny".to_string());
    dictionary.insert("stew".to_string());
    dictionary.insert("design".to_string());
    dictionary.insert("control".to_string());

    dictionary
}
