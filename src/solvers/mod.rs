pub mod basicsolver;
pub mod dumbsolver;
pub mod greedysolver;
pub mod vibexsolver;

/*

enum Feedback { Black, Yellow, Green }

Feedback::on(guess: &str, target: &str) -> [Feedback; 5]
- Returns the Wordle-style feedback for `guess` against `target`.

Feedback::signature(fb: &[Feedback; 5]) -> usize
- Encodes a 5‑tile feedback pattern into a number in 0..243.

trait Solver {
	fn new(word_bank: &[&'static str]) -> Self;
	- Creates a solver with the full allowed word list.

	fn guess(&mut self) -> &'static str;
	- Returns the solver's next guess.

	fn feedback(&mut self, guess: &str, fb: [Feedback; 5]);
	- Updates the solver with the feedback from its last guess.
}

*/