use std::collections::HashSet;
use std::io::{self, Write};

mod game;
mod messages;

// Req 9
// This program interacts with the user via the terminal

fn main() {
    let dictionary = construct_dictionary();
    
    loop {
        // Set up the game and play
        
        // Req 2
        // Req 8a
        // Pick a word
        let target_word = choose_word(&dictionary);
        let game = game::HangmanGame::new(&target_word);
        let result = game.play();

        match result {
            game::GameResult::Success(guesses) => {
                // Req 7
                println!("{}", messages::game_success(&target_word, guesses));
            },
            game::GameResult::Failure {
                    ..
                } => todo!("Let the user know they lost the game")
        }

        // Req 8b
        // Ask the user if they want to play again
        // If they don't, quit the game
        if !play_again() {
            break;
        }
    }

    // Req 8b
    // Thank the user for playing and quit the game
    println!("{}", messages::END_GAME);

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

fn choose_word(dictionary: &HashSet<String>) -> String {
    // The randomly chosen index of the word
	let idx = rand::random::<usize>() % dictionary.len();

    // Get the `n`th word from the dictionary
    // 
    // This can safely be unwrapped because the index
    // `idx` will always be less than the length of the
    // dictionary
	dictionary.iter().nth(idx).unwrap().to_string()
}

fn play_again() -> bool {
    // Req 8
    // Ask if they would like to play again
    print!("{}", messages::PLAY_AGAIN_PROMPT);io::stdout().flush().expect("stdout not available"); // Flush stdout

    // Read in the user's character
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("stdin not available");
    println!();

    // Convert the character from String to boolean or report error
    // Also, converts the character to lowercase so that 'Y', 'y', 'N',
    // and 'n' are all accepted
    let user_char = user_input.trim().parse::<char>();
    match user_char.map(|c| c.to_ascii_lowercase()) {
        Ok('y') => true,
        Ok('n') => false,
        _ => {
            println!("{}\n", messages::INVALID_INPUT);
            play_again()
        }
    }
}
