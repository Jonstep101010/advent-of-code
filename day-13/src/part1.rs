use glam::UVec2;
use miette::miette;
use nom::{
	IResult, Parser,
	bytes::complete::tag,
	character::complete::{self, line_ending},
	multi::separated_list1,
	sequence::{preceded, separated_pair, terminated, tuple},
};

pub type TokenCost = u64;
const MAX_BTN_PRESSES: u32 = 100;
const COST_A: u32 = 3;
const COST_B: u32 = 1;

#[derive(Debug)]
struct ClawMachine {
	a: UVec2,
	b: UVec2,
	prize: UVec2,
}

fn claw_prize(/* location information */) -> Option<TokenCost> {
	let mut presses_a = 0;
	let mut presses_b = 0;

	if presses_a <= MAX_BTN_PRESSES && presses_b <= MAX_BTN_PRESSES {
		// if possible in a, b <= 100
		Some(presses_a as u64 + presses_b as u64)
	} else {
		// no possibility of reaching prize
		None
	}
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

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	// parse input into usable data
	// x and y for each button, prize location
	// IVec2?
	let (_, games) = parse(input).map_err(|err| miette!("invalid blocks in input: {}", err))?;
	dbg!(&games[0]);
	// use pathfinding to map with button combinations
	// (max 100 presses per button)
	// check which are possible: Some(token_cost), None (not possible)
	// sum up all possible prizes' token cost

	let results = games.iter().map(|machine| {
		// do something with djikstra algo
		// example usage:
		static GOAL: (i32, i32) = (4, 6);
		let result = pathfinding::prelude::dijkstra(&(1, 1),/* successors */
							|&(x, y)| vec![(x+1,y+2), (x+1,y-2), (x-1,y+2), (x-1,y-2),
											(x+2,y+1), (x+2,y-1), (x-2,y+1), (x-2,y-1)]
										.into_iter().map(|p| (p, 1)),
							|&p| p == GOAL);
		assert_eq!(result.expect("no path found").1, 4);
		// end example
		let result = pathfinding::prelude::dijkstra(&UVec2::ZERO,
		/* somehow define moves */ , |prize| {*prize == machine.prize});
	});

	// minimum tokens to get all possible prizes
	let required_tokens: u64 = 0;
	Ok(required_tokens.to_string())
}

#[cfg(test)]
mod tests {
	use miette::Error;

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
	// 	#[test]
	// 	fn test_second_impossible() -> miette::Result<()> {
	// 		// there are no possible combinations for reaching the prize
	// 		let input = "Button A: X+26, Y+66
	// Button B: X+67, Y+21
	// Prize: X=12748, Y=12176";
	// 		assert_eq!(mietteError, claw_prize(input)?);
	// 		Ok(())
	// 	}
}
