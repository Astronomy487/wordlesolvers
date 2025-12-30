pub struct DumbSolver;
impl crate::Solver for DumbSolver {
	fn new(_: &[&'static str]) -> Self {
		DumbSolver
	}
	fn guess(&mut self) -> &'static str {
		"hover"
	}
	fn feedback(&mut self, _the_guess: &'static str, _feedback: [crate::Feedback; 5]) {}
}
