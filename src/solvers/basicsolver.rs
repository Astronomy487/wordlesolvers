pub struct BasicSolver {
	bank: std::collections::HashSet<&'static str>
}
impl crate::Solver for BasicSolver {
	fn new(word_bank: &[&'static str]) -> Self {
		BasicSolver {
			bank: word_bank.iter().copied().collect()
		}
	}
	fn guess(&mut self) -> &'static str {
		for guess in &self.bank {
			return guess;
		}
		"BAD"
	}
	fn feedback(&mut self, the_guess: &'static str, feedback: [crate::Feedback; 5]) {
		self.bank
			.retain(|possible_target| crate::Feedback::on(the_guess, possible_target) == feedback);
	}
}
