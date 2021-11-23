use std::collections::HashSet;

mod messages;

/// A struct representation of the "Hangman" game
pub struct HangmanGame {
	target_word: String,
	user_guesses: HashSet<char>,
}

impl HangmanGame {
	/// Creates a new game based on a given word
	pub fn new(target_word: String) -> Self {
		Self {
			target_word,
			user_guesses: HashSet::new(),
		}
	}

	/// Play the game, returning the result of playing the
	/// game (either a success or a failure)
	pub fn play(self) -> GameResult {
		// Req 1
		// Welcome the user
		println!("{}", messages::WELCOME);
		println!();

		todo!("Build the game loop")
	}
}

/// An enum that represents the result of playing a game
pub enum GameResult {
	Success(u32),
	Failure {
		target_word: String,
		unguessed_chars: HashSet<char>,
	}
}