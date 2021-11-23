use std::collections::HashSet;

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
}

/// An enum that represents the result of playing a game
pub enum GameResult {
	Success(u32),
	Failure {
		target_word: String,
		unguessed_chars: HashSet<char>,
	}
}