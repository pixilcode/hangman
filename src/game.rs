use std::collections::HashSet;
use std::io::{self, Write};

mod messages;

/// A struct representation of the "Hangman" game
pub struct HangmanGame {
	target_word: String,
	correct_guesses: HashSet<char>,
	incorrect_guesses: HashSet<char>,
}

impl HangmanGame {
	/// Creates a new game based on a given word
	pub fn new(target_word: String) -> Self {
		Self {
			target_word,
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
	fn game_loop(self) -> GameResult {
		if false /* todo!("Determine if all the letters have been guessed") */ {
			// Req 6
			// End condition: all the letters have been guessed
			todo!("Return a success result with the number of guesses")
		} else if "a" == "b" /* todo!("Determine if the player has guessed incorrectly too many times") */ {
			// End condition: the player has incorrectly guessed too many times
			todo!("Return a failure result with the number of incorrect guesses")
		} else {
			// Req 3
			// Display the word with correctly guessed chars filled in
			println!("{} ({} letters)", self.show_word(), self.target_word.len());

			// Req 4
			// Ask the user to guess a letter
			print!("{}", messages::GUESS_LETTER_PROMPT);
			io::stdout().flush().expect("stdout not available"); // Flush stdout

			// Read in the user's character
			let mut user_input = String::new();
			io::stdin().read_line(&mut user_input).expect("stdin not available");

			// Convert the input from String to char or report error
			let user_char = user_input.trim().parse::<char>();
			match user_char {
				Ok(c) => {
					todo!("Handle user char")
				},
				Err(_) => {
					println!("{}", messages::INVALID_INPUT);
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
	Success(u32),
	Failure {
		target_word: String,
		unguessed_chars: HashSet<char>,
	}
}