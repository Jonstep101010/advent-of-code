use std::collections::HashSet;

use nom::{
	IResult, Parser,
	character::complete::{self, line_ending, space1},
	multi::{fold_many1, separated_list1},
	sequence::delimited,
};

fn validate_button_sequences(machine: Machine) -> i32 {
	dbg!(&machine);
	let mut machine_set = HashSet::new();
	assert!(machine_set.insert(vec![false; machine.goal_indicator_seq.len()]));
	let mut machine_total = 0;
	loop {
		machine_set = machine_set
			.into_iter()
			.flat_map(|state| {
				machine.button_seq.iter().map(move |button| {
					let mut new_state = state.clone();
					for bit in button {
						let val = new_state.get_mut(*bit as usize).unwrap();
						*val = !*val
					}
					new_state
				})
			})
			.collect();
		machine_total += 1;
		if machine_set.contains(&machine.goal_indicator_seq) {
			return machine_total;
		}
	}
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
	joltage_seq: Vec<u8>,
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

fn parse_joltage_seq(input: &str) -> IResult<&str, Vec<u8>> {
	delimited(
		complete::char('{'),
		separated_list1(complete::char(','), complete::u8),
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
