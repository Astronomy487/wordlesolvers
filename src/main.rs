#![deny(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![deny(unused_mut, unused_results, unused_variables, unused_imports)]
#![deny(unsafe_code, unused_unsafe)]
#![deny(unreachable_code, unreachable_patterns)]
#![allow(dead_code)]
#![deny(private_interfaces)]
#![deny(absolute_paths_not_starting_with_crate)]

static WORDLIST: &str = include_str!("wordlist.txt");

#[derive(Copy, Clone, PartialEq)]
pub enum Feedback {
	Black,
	Yellow,
	Green
}
impl Feedback {
	fn on(guess: &'static str, target: &'static str) -> [Feedback; 5] {
		let mut result = [Feedback::Black; 5];
		let guess_chars: Vec<char> = guess.chars().collect();
		let target_chars: Vec<char> = target.chars().collect();
		let mut remaining = std::collections::HashMap::<char, usize>::new();
		for &c in &target_chars {
			*remaining.entry(c).or_insert(0) += 1;
		}
		for i in 0..5 {
			if guess_chars[i] == target_chars[i] {
				result[i] = Feedback::Green;
				let entry = remaining.get_mut(&guess_chars[i]).unwrap();
				*entry -= 1;
			}
		}
		for i in 0..5 {
			if result[i] == Feedback::Green {
				continue;
			}
			let c = guess_chars[i];
			if let Some(count) = remaining.get_mut(&c) {
				if *count > 0 {
					result[i] = Feedback::Yellow;
					*count -= 1;
				}
			}
		}
		result
	}
	fn signature(feedback: &[Feedback; 5]) -> usize {
		// returns something in [0, 243)
		fn letter_signature(it: &Feedback) -> usize {
			match it {
				Feedback::Black => 0,
				Feedback::Yellow => 1,
				Feedback::Green => 2
			}
		}
		letter_signature(&feedback[4])
			+ 3 * letter_signature(&feedback[3])
			+ 3 * 3 * letter_signature(&feedback[2])
			+ 3 * 3 * 3 * letter_signature(&feedback[1])
			+ 3 * 3 * 3 * 3 * letter_signature(&feedback[0])
	}
}

pub trait Solver {
	fn new(word_bank: &[&'static str]) -> Self;
	fn guess(&mut self) -> &'static str;
	fn feedback(&mut self, the_guess: &'static str, feedback: [crate::Feedback; 5]);
}

fn evaluate<S: Solver>(words: &[&'static str], times_over: usize) {
	let name = std::any::type_name::<S>().rsplit("::").next().unwrap();
	let eta_nanos = |nanos: usize| {
		print!("\r\x1b[2K");
		print!("Evaluating \x1b[96m{}\x1b[0m...", name);
		if nanos > 0 {
			print!(" (eta {}s)", nanos / 1_000_000_000);
		}
		std::io::Write::flush(&mut std::io::stdout()).unwrap();
	};
	let mut distribution = [0; 7];
	let mut elapsed = Vec::new();
	fn elapsed_average_nanos(elapsed: &Vec<std::time::Duration>) -> usize {
		(elapsed
			.iter()
			.map(std::time::Duration::as_nanos)
			.sum::<u128>()
			/ (elapsed.len() as u128)) as usize
	}
	eta_nanos(0);
	let mut hard_mode_compliant = true;
	for _ in 0..times_over {
		for &target in words {
			let mut set_of_possible_words = if hard_mode_compliant {
				words.iter().copied().collect::<std::collections::HashSet<_>>()
			} else {
				std::collections::HashSet::new()
			};
			let start = std::time::Instant::now();
			let mut solver = S::new(words);
			let mut attempts = 0;
			loop {
				attempts += 1;
				let guess = solver.guess();
				if !words.contains(&guess) {
					println!(
						"\r\x1b[2K\x1b[96m{}\x1b[0m could not be evaluated as it guessed invalid word \"{}\"\n",
						name, guess
					);
					return;
				}
				if hard_mode_compliant && !set_of_possible_words.contains(guess) {
					hard_mode_compliant = false;
				}
				let fb = Feedback::on(guess, target);
				if hard_mode_compliant {
					set_of_possible_words.retain(|&candidate| crate::Feedback::on(guess, candidate) == fb);
				}
				solver.feedback(guess, fb);
				if guess == target {
					distribution[attempts - 1] += 1;
					break;
				}
				if attempts >= 6 {
					distribution[6] += 1;
					break;
				}
			}
			elapsed.push(start.elapsed());
			if elapsed.len() % 1000 == 0 || elapsed.len() < 30 {
				let runs_left = words.len() * times_over - elapsed.len();
				if runs_left > 0 {
					let nanos_left = elapsed_average_nanos(&elapsed) * runs_left;
					eta_nanos(nanos_left);
				}
			}
		}
	}
	print!("\r\x1b[2K");
	let left_column: [&str; 7] = [
		name,
		&format!("avg {}us ", elapsed_average_nanos(&elapsed) / 1000),
		&format!("{}/{}", elapsed.len() - distribution[6], elapsed.len()),
		&format!(
			"{}% success{}",
			(elapsed.len() - distribution[6]) * 100 / elapsed.len(),
			if distribution[6] == 0 { " :)" } else { "" }
		),
		if hard_mode_compliant {"hard mode"} else {""},
		"",
		""
	];
	let mut report = String::new();
	for (i, amount) in distribution.iter().enumerate() {
		report += &format!(
			"{}{:<18}\x1b[0m{:<4}{}{:>6} ",
			match i {
				0 => "\x1b[96m",
				_ => "\x1b[90m"
			},
			left_column[i],
			match i {
				6 => "fail",
				more => &(more+1).to_string()
			},
			match i {
				0 => "\x1b[38;2;44;179;66m",
				1 => "\x1b[38;2;97;204;88m",
				2 => "\x1b[38;2;146;217;53m",
				3 => "\x1b[38;2;217;212;53m",
				4 => "\x1b[38;2;211;168;49m",
				5 => "\x1b[38;2;204;121;43m",
				6 => "\x1b[38;2;204;43;44m",
				_ => unreachable!()
			},
			amount
		);
		let mut eighths_to_do = amount / (4 * times_over);
		if *amount > 0 && eighths_to_do == 0 {
			eighths_to_do = 1;
		}
		for _ in 0..eighths_to_do/8 {
			report += &format!("█");
		}
		report += ["", "▎", "▍", "▌", "▋", "▊", "▉", "█"][eighths_to_do % 8];
		report += &format!("\x1b[0m\n");
	}
	println!("{}", report);
}

mod solvers;

fn main() {
	let times_over = std::env::args()
		.nth(1)
		.unwrap_or("1".to_string())
		.parse()
		.unwrap_or(1);

	let words: Vec<&'static str> = WORDLIST.lines().collect();
	evaluate::<solvers::dumbsolver::DumbSolver>(&words, times_over);
	evaluate::<solvers::basicsolver::BasicSolver>(&words, times_over);
	evaluate::<solvers::greedysolver::GreedySolver>(&words, times_over);
}
