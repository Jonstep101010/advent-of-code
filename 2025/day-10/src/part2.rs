use itertools::Itertools;
use nom::{
	IResult, Parser,
	character::complete::{self, line_ending, space1},
	multi::{fold_many1, separated_list1},
	sequence::delimited,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;

fn button_combination_patterns(
	coeffs: Vec<Vec<bool>>,
	num_indicators: usize,
) -> HashMap<Vec<bool>, HashMap<Vec<u32>, usize>> {
	// Initialize all 2^num_indicators possible parity combinations upfront
	let mut patterns_by_parity: HashMap<Vec<bool>, HashMap<Vec<u32>, usize>> = (0..1
		<< num_indicators)
		.map(|mask| {
			let parity: Vec<bool> = (0..num_indicators)
				.map(|i| (mask & (1usize << i)) != 0)
				.collect();
			(parity, HashMap::new())
		})
		.collect();

	let num_buttons = coeffs.len();

	// Try all 2^n subsets of buttons (represented as combinations)
	for num_buttons_count in 0..=num_buttons {
		for button_indices in (0..num_buttons).combinations(num_buttons_count) {
			// Sum coefficients for selected buttons (converting bool to u32)
			let effect: Vec<u32> = (0..num_indicators)
				.map(|j| button_indices.iter().map(|&i| coeffs[i][j] as u32).sum())
				.collect();

			// Calculate parity pattern from summed effects
			let parity: Vec<bool> = effect.iter().map(|&e| e % 2 != 0).collect();

			// Store minimal press count for this effect
			patterns_by_parity
				.get_mut(&parity)
				.unwrap()
				.entry(effect)
				.or_insert(num_buttons_count);
		}
	}

	patterns_by_parity
}

thread_local! {
	static SOLVE_CACHE: std::cell::RefCell<HashMap<Vec<u32>, usize>> = std::cell::RefCell::new(HashMap::new());
}

fn solve_impl(
	goal: Vec<u32>,
	patterns_by_parity: &HashMap<Vec<bool>, HashMap<Vec<u32>, usize>>,
) -> usize {
	if goal.iter().all(|&g| g == 0) {
		return 0;
	}

	let cached_result = SOLVE_CACHE.with(|cache| cache.borrow().get(&goal).copied());
	if let Some(result) = cached_result {
		return result;
	}

	let mut min_presses = usize::MAX;
	let parity: Vec<bool> = goal.iter().map(|&g| g % 2 != 0).collect();

	if let Some(phase1_candidates) = patterns_by_parity.get(&parity) {
		for (p1_effect, p1_presses) in phase1_candidates {
			if itertools::all(std::iter::zip(p1_effect, &goal), |(e, g)| e <= g) {
				let p2_goal: Vec<u32> = std::iter::zip(p1_effect, &goal)
					.map(|(e, g)| (g - e) / 2)
					.collect();
				let p2_presses = solve_impl(p2_goal, patterns_by_parity);
				if p2_presses == usize::MAX {
					// No valid solution for phase 2, skip this candidate
					continue;
				}
				let total = *p1_presses + 2 * p2_presses;
				min_presses = min_presses.min(total);
			}
		}
	}

	SOLVE_CACHE.with(|cache| {
		cache.borrow_mut().insert(goal, min_presses);
	});
	min_presses
}

fn validate_button_sequences(machine: &Machine) -> usize {
	let num_indicators = machine.joltage_seq.len();

	// Convert buttons to coefficient matrix (1 if button affects indicator, 0 otherwise)
	let coeffs: Vec<Vec<bool>> = machine
		.button_seq
		.iter()
		.map(|button| {
			(0..num_indicators)
				.map(|i| button.contains(&(i as u8)))
				.collect()
		})
		.collect();

	// Precompute all reachable patterns and their minimal press counts (cached across machines)
	let patterns_by_parity = button_combination_patterns(coeffs, num_indicators);

	// Convert goal to Vec<u32>
	let goal: Vec<u32> = machine.joltage_seq.iter().map(|&x| x as u32).collect();

	// Clear cache for new machine
	SOLVE_CACHE.with(|cache| {
		cache.borrow_mut().clear();
	});

	solve_impl(goal, &patterns_by_parity)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let (_, machines) = parse(input).expect("fine");
	let reaching_buttons = machines
		.par_iter()
		.map(validate_button_sequences)
		.sum::<usize>();
	Ok(reaching_buttons.to_string())
}

#[derive(Debug)]
struct Machine {
	// goal_indicator_seq: Vec<bool>,
	button_seq: Vec<Vec<u8>>,
	joltage_seq: Vec<u16>,
}

fn parse(input: &str) -> IResult<&str, Vec<Machine>> {
	nom::multi::separated_list1(line_ending, |single_input| {
		let (single_input, _goal_indicator_seq) = parse_goal_seq(single_input)?;
		// repeatedly skip spaces
		let (single_input, _) = complete::space1(single_input)?;
		// button parser with spaces
		let (single_input, button_seq) =
			separated_list1(space1, parse_button_seq).parse(single_input)?;
		let (single_input, _) = complete::space1(single_input)?;
		// joltage parser
		let (single_input, joltage_seq) = parse_joltage_seq(single_input)?;

		Ok((
			single_input,
			Machine {
				// goal_indicator_seq,
				button_seq,
				joltage_seq,
			},
		))
	})
	.parse(input)
}
fn parse_goal_seq(input: &str) -> IResult<&str, Vec<bool>> {
	delimited(
		complete::char('['),
		fold_many1(
			nom::branch::alt((complete::char('.'), complete::char('#'))),
			Vec::new,
			|mut acc: Vec<bool>, state| {
				acc.push(match state {
					'.' => false,
					'#' => true,
					_ => panic!(),
				});
				acc
			},
		),
		complete::char(']'),
	)
	.parse(input)
}
fn parse_button_seq(input: &str) -> IResult<&str, Vec<u8>> {
	delimited(
		complete::char('('),
		separated_list1(complete::char(','), complete::u8),
		complete::char(')'),
	)
	.parse(input)
}

fn parse_joltage_seq(input: &str) -> IResult<&str, Vec<u16>> {
	delimited(
		complete::char('{'),
		separated_list1(complete::char(','), nom::character::complete::u16),
		complete::char('}'),
	)
	.parse(input)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";
		assert_eq!("33", process(input)?);
		Ok(())
	}
}
