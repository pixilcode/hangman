use std::collections::HashSet;

/// A struct representation of the "Hangman" game
pub struct HangmanGame {
	target_word: String,
	user_guesses: HashSet<char>,
}

/// An enum that represents the result of playing a game
pub enum GameResult {
	Success(u32),
	Failure {
		target_word: String,
		unguessed_chars: HashSet<char>,
	}
}