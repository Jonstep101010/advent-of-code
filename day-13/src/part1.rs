use glam::UVec2;
use miette::miette;
use nom::{
	IResult, Parser,
	bytes::complete::tag,
	character::complete::{self, line_ending},
	multi::separated_list1,
	sequence::{preceded, separated_pair, terminated, tuple},
};

const COST_A: u32 = 3;
const COST_B: u32 = 1;

#[derive(Debug)]
struct ClawMachine {
	a: UVec2,
	b: UVec2,
	prize: UVec2,
}

fn parse_button(input: &str) -> IResult<&str, UVec2> {
	preceded(
		{
			if input.chars().nth(7).unwrap() == 'A' {
				tag("Button A: X+")
			} else {
				tag("Button B: X+")
			}
		},
		separated_pair(complete::u32, tag(", Y+"), complete::u32).map(|(x, y)| UVec2::new(x, y)),
	)(input)
}

fn parse_prize(input: &str) -> IResult<&str, UVec2> {
	preceded(
		tag("Prize: X="),
		separated_pair(complete::u32, tag(", Y="), complete::u32).map(|(x, y)| UVec2::new(x, y)),
	)(input)
}

// block in input:
// Button A: X+77, Y+52
// Button B: X+14, Y+32
// Prize: X=5233, Y=14652
// parse each blocks' data individually: by '\n'
fn parse<'a>(input: &'a str) -> IResult<&'a str, Vec<ClawMachine>> {
	/*
	I wanted to try implementing a closure, that's why we needed lifetimes
	the function implementation of the closure works without them:
	fn game_fn(input: &str) -> IResult<&str, ClawMachine> {
		let (input, (a, b, prize)) = tuple((
			terminated(parse_button_a, line_ending),
			terminated(parse_button_b, line_ending),
			parse_prize,
		))(input)?;
		Ok((input, ClawMachine { a, b, prize }))
	}
	*/
	let game =
		|input: &'a str| -> Result<(&'a str, ClawMachine), nom::Err<nom::error::Error<&str>>> {
			let (input, (a, b, prize)) = tuple((
				terminated(parse_button, line_ending),
				terminated(parse_button, line_ending),
				parse_prize,
			))(input)?;
			Ok((input, ClawMachine { a, b, prize }))
		};
	separated_list1(tuple((line_ending, line_ending)), game)(input)
}

/// use pathfinding to map with button combinations
/// (max 100 presses per button) - works without manual implementation
/// check which are possible: Some(token_cost), None (not possible) - dijkstra handles this
fn claw_prize_cost(games: &[ClawMachine]) -> Vec<u32> {
	games
		.iter()
		.filter_map(|machine| {
			let result = pathfinding::prelude::dijkstra(
				&UVec2::ZERO, // start pos
				|&position| {
					// set successors for goal_pos
					if position.x > machine.prize.x || position.y > machine.prize.y {
						// went past goal, non-success
						vec![]
					} else {
						// walk path
						vec![
							(position + machine.a, COST_A),
							(position + machine.b, COST_B),
						]
					}
				},
				|prize| *prize == machine.prize,
			);
			// why does this not need an unwrap?
			result.map(|res_item| res_item.1)
		})
		.collect()
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	// parse input into usable data
	// x and y for each button, prize location - UVec2(x, y)
	let (_, games) = parse(input).map_err(|err| miette!("invalid blocks in input: {}", err))?;

	// token costs of possible paths in vector
	let results = claw_prize_cost(&games);

	// minimum tokens to get all possible prizes
	// sum up all possible prizes' token cost
	let required_tokens: u32 = results.iter().sum();
	Ok(required_tokens.to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
		assert_eq!("480", process(input)?);
		Ok(())
	}
	#[test]
	fn test_first_example() -> miette::Result<()> {
		let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400";
		assert_eq!("280", process(input)?);
		Ok(())
	}
	#[test]
	fn test_second_impossible() -> miette::Result<()> {
		// there are no possible combinations for reaching the prize
		let input = "Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176
";
		assert_eq!("0", process(input)?);
		Ok(())
	}
}
