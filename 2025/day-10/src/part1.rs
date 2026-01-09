use std::collections::HashMap;

use nom::{
	IResult, Parser,
	character::complete::{self, line_ending, space1},
	multi::{fold_many1, separated_list1},
	sequence::delimited,
};

fn button_combination_patterns(
	coeffs: &[Vec<bool>],
	num_indicators: usize,
) -> HashMap<Vec<bool>, usize> {
	let mut patterns = HashMap::new();
	let num_buttons = coeffs.len();

	// Try all 2^n subsets of buttons (represented as bitmasks)
	for mask in 0u32..(1 << num_buttons) {
		// Build the XOR effect of all buttons enabled in `mask` using a fold
		let effect = (0..num_buttons)
			.filter(|&idx| (mask & (1u32 << idx)) != 0)
			.fold(vec![false; num_indicators], |mut acc, idx| {
				acc.iter_mut()
					.zip(&coeffs[idx])
					.for_each(|(e, &bit)| *e ^= bit);
				acc
			});

		// Store minimal press count for this effect pattern
		let num_presses = mask.count_ones() as usize;
		patterns
			.entry(effect)
			.and_modify(|existing: &mut usize| *existing = (*existing).min(num_presses))
			.or_insert(num_presses);
	}

	patterns
}

fn validate_button_sequences(machine: Machine) -> i32 {
	let num_indicators = machine.goal_indicator_seq.len();

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

	// Precompute all reachable patterns and their minimal press counts
	button_combination_patterns(&coeffs, num_indicators)
		.get(&machine.goal_indicator_seq)
		.copied()
		.unwrap_or(usize::MAX) as i32
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let (_, machines) = parse(input).expect("fine");
	let mut reaching_buttons = 0;
	for m in machines {
		reaching_buttons += validate_button_sequences(m);
	}
	Ok(reaching_buttons.to_string())
}

#[derive(Debug)]
struct Machine {
	goal_indicator_seq: Vec<bool>,
	button_seq: Vec<Vec<u8>>,
	joltage_seq: Vec<u16>,
}

fn parse(input: &str) -> IResult<&str, Vec<Machine>> {
	nom::multi::separated_list1(line_ending, |single_input| {
		let (single_input, goal_indicator_seq) = parse_goal_seq(single_input)?;
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
				goal_indicator_seq,
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
		assert_eq!("7", process(input)?);
		Ok(())
	}
}
