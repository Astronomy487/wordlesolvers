pub struct GreedySolver {
	bank: std::collections::HashSet<&'static str>,
	used_initial_guess_yet: bool
}

impl GreedySolver {
	fn utility_for_guess(&self, guess: &'static str) -> usize {
		let mut counts = vec![0usize; 243];

		for &truth in &self.bank {
			let fb = crate::Feedback::on(guess, truth);
			let sig = crate::Feedback::signature(&fb);
			counts[sig] += 1;
		}

		counts.iter().map(|c| c * c).sum()
	}

	fn best_guess_internal(&self) -> &'static str {
		let mut best_word = "";
		let mut best_util = usize::MAX;

		for &guess in &self.bank {
			let util = self.utility_for_guess(guess);
			if util < best_util {
				best_util = util;
				best_word = guess;
			}
		}

		best_word
	}
}

impl crate::Solver for GreedySolver {
	fn new(word_bank: &[&'static str]) -> Self {
		GreedySolver {
			bank: word_bank.iter().copied().collect(),
			used_initial_guess_yet: false
		}
	}
	fn guess(&mut self) -> &'static str {
		if !self.used_initial_guess_yet {
			self.used_initial_guess_yet = true;
			return "arise"; // TODO LATER maybe one-time precomputation? for now hardcoded
		}
		self.best_guess_internal()
	}
	fn feedback(&mut self, guess: &'static str, fb: [crate::Feedback; 5]) {
		self.bank
			.retain(|&candidate| crate::Feedback::on(guess, candidate) == fb);
	}
}
