pub struct VibexSolver {
	bank: std::collections::HashSet<&'static str>,
	guesses_made: u8
}
impl crate::Solver for VibexSolver {
	fn new(word_bank: &[&'static str]) -> Self {
		VibexSolver {
			bank: word_bank.iter().copied().collect(),
			guesses_made: 0
		}
	}
	fn guess(&mut self) -> &'static str {
		self.guesses_made += 1;
		match self.guesses_made {
			1 => "waltz",
			2 => "vibex",
			3 => "gymps",
			4 => "fjord",
			5 => "chunk",
			_ => {
				for guess in &self.bank {
					return guess;
				}
				"BAD"
			}
		}
	}
	fn feedback(&mut self, the_guess: &'static str, feedback: [crate::Feedback; 5]) {
		self.bank
			.retain(|possible_target| crate::Feedback::on(the_guess, possible_target) == feedback);
	}
}
