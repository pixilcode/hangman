use std::collections::HashSet;

mod game;
mod messages;

fn main() {
    // Req 2
    // Pick a word
    let dictionary = construct_dictionary();

    // Set up the game and play
    let target_word = choose_word(dictionary);
    let game = game::HangmanGame::new(target_word);
    let result = game.play();
    
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

fn choose_word(dictionary: HashSet<String>) -> String {
    // The randomly chosen index of the word
	let idx = rand::random::<usize>() % dictionary.len();

    // Get the `n`th word from the dictionary
    // 
    // This can safely be unwrapped because the index
    // `idx` will always be less than the length of the
    // dictionary
	dictionary.iter().nth(idx).unwrap().to_string()
}
