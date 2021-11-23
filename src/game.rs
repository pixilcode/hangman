use std::collections::HashSet;
use std::io::{self, Write};
use crate::messages;

/// A struct representation of the "Hangman" game
pub struct HangmanGame {
	target_word: String,
	correct_guesses: HashSet<char>,
	incorrect_guesses: HashSet<char>,
}

impl HangmanGame {
	/// Creates a new game based on a given word
	pub fn new(target_word: &str) -> Self {
		Self {
			target_word: target_word.to_string(),
			correct_guesses: HashSet::new(),
			incorrect_guesses: HashSet::new(),
		}
	}

	/// Play the game, returning the result of playing the
	/// game (either a success or a failure)
	pub fn play(self) -> GameResult {
		// Req 1
		// Welcome the user
		println!("{}", messages::WELCOME);
		println!();

		self.game_loop()
	}

	/// A recursive function representing the game loop
	fn game_loop(mut self) -> GameResult {
		if self.target_word.chars().all( // TODO: Make this more efficient
			|c| self.correct_guesses.contains(&c)
		) {
			// Req 6
			// End if all the letters were correctly guessed
			// End condition: all the letters have been guessed
			GameResult::Success(self.correct_guesses.len() + self.incorrect_guesses.len())
		} else if "a" == "b" /* todo!("Determine if the player has guessed incorrectly too many times") */ {
			// End condition: the player has incorrectly guessed too many times
			todo!("Return a failure result with the number of incorrect guesses")
		} else {
			// Req 3
			// Req 4a
			// Display the word with correctly guessed chars filled in
			println!("{} ({} letters)\n", self.show_word(), self.target_word.len());

			// Req 4
			// Ask the user to guess a letter
			print!("{}", messages::GUESS_LETTER_PROMPT);
			io::stdout().flush().expect("stdout not available"); // Flush stdout

			// Read in the user's character
			let mut user_input = String::new();
			io::stdin().read_line(&mut user_input).expect("stdin not available");
			println!();

			// Convert the input from String to char or report error
			let user_char = user_input.trim().parse::<char>();
			match user_char {
				Ok(c) => {
					// Req 4b
					// Let the user know if they guessed correctly
					if self.target_word.contains(c) {
						println!("{}\n", messages::correct_guess(c));
						self.correct_guesses.insert(c);
					} else {
						println!("{}\n", messages::incorrect_guess(c));
						self.incorrect_guesses.insert(c);
					}

					// Req 4b
					// Req 5
					// Display the total number of guesses as well
					// as the number of correct and incorrect guesses
					println!("{}\n",
						messages::display_guesses(
							&self.correct_guesses,
							&self.incorrect_guesses
						)
					);

					self.game_loop()
				},
				Err(_) => {
					println!("{}\n", messages::INVALID_INPUT);
					println!();
					self.game_loop()
				}
			}
		}
		
	}

	/// Shows the target_word with unguessed chars replaced with a `_`
	fn show_word(&self) -> String {
		self.target_word.chars().map(
			|c| if self.correct_guesses.contains(&c) {
				c
			} else {
				'_'
			}
		).collect()
	}
}

/// An enum that represents the result of playing a game
pub enum GameResult {
	Success(usize),
	Failure {
		target_word: String,
		unguessed_chars: HashSet<char>,
	}
}