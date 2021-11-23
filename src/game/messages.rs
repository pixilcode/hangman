use std::collections::HashSet;

pub const WELCOME: &str = "\
Welcome to Hangman! We're glad to have you.
Ready to begin?";

pub const GUESS_LETTER_PROMPT: &str = "Please enter your guess: ";

pub const INVALID_INPUT: &str = "Your input was not valid. Please input a single character";

pub fn correct_guess(guess: char) -> String {
	format!("'{}' *is* in the word!", guess)
}

pub fn incorrect_guess(guess: char) -> String {
	format!("'{}' is *not* in the word!", guess)
}

pub fn display_guesses(correct_guesses: &HashSet<char>, incorrect_guesses: &HashSet<char>) -> String {
	format!(
		"\
Correct guesses: {}
Incorrect guesses: {}
Total guesses: {} (correct: {}, incorrect: {})",
		hash_set_to_string(correct_guesses),
		hash_set_to_string(incorrect_guesses),
		correct_guesses.len() + incorrect_guesses.len(),
		correct_guesses.len(),
		incorrect_guesses.len()
	)
}

/// Print out a hashset in the form of `"a, b, ..., e"`
fn hash_set_to_string(set: &HashSet<char>) -> String {
	let mut chars = set.iter().collect::<Vec<_>>(); // Turn it into a vetor to be sorted
	chars.sort();
	chars.iter().enumerate().map( // Enumerate it
		|(idx, c)| if idx == set.len() - 1 {
			c.to_string() // If it's the last one, don't add a comma
		} else {
			format!("{}, ", c) // Otherwise, add on a comma
		}
	).collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_hs_to_string() {
		let mut hs = HashSet::new();
		hs.insert('c');
		hs.insert('b');
		hs.insert('a');

		// Sorted in alphabetic order
		assert_eq!("a, b, c", hash_set_to_string(&hs));
	}
}