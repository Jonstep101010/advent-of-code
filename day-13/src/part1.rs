use glam::UVec2;
use nom::{
	IResult, Parser,
	bytes::complete::tag,
	character::complete::{self, line_ending},
	multi::separated_list1,
	sequence::{preceded, separated_pair, terminated},
};

pub type TokenCost = u64;
const MAX_BTN_PRESSES: u32 = 100;

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
	if input.contains('A') {
		let tag_button = tag("Button A: X+");
	} else {
		let tag_button = tag("Button B: X+");
	}
	preceded(
		tag_button,
		separated_pair(complete::u32, tag(", Y+"), complete::u32),
	)(input)
	.map(|(input, (x, y))| UVec2::new(x, y))
}

fn parse_prize() -> IResult<&str, UVec2> {}

// input:
// instruc tion: X+val, Y+val
// btninstruct = (x, y) = &line[12..(location_comma - 1)], &line[(location_comma + 4)..(line.len())]
// Button A: X+77, Y+52
// Button B: X+14, Y+32
// instruc tion: X=val, Y=val
// prize_loc = (x, y) = &line[9..(location_comma - 1)], &line[(location_comma + 4)..(line.len())]
// Prize: X=5233, Y=14652
// parse each blocks' data individually: by '\n'
fn parse(input: &str) -> IResult<&str, Vec<ClawMachine>> {
	let games = vec![];
	let game = {
		let (_, (a, b, prize)) = tuple(
			terminated(parse_button, line_ending),
			terminated(parse_button, line_ending),
			terminated(parse_prize, line_ending),
		)(input)?;
		ClawMachine { a, b, prize }
	};
	separated_list1(line_ending, game)(input)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	// parse input into usable data
	// x and y for each button, prize location
	// IVec2?

	// use pathfinding to map with button combinations
	// (max 100 presses per button)
	// check which are possible: Some(token_cost), None (not possible)
	// sum up all possible prizes' token cost

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
