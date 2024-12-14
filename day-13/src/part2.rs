#![warn(clippy::pedantic)]

use glam::{DMat2, U64Vec2};
use miette::miette;
use nom::{
	IResult, Parser,
	bytes::complete::tag,
	character::complete::{self, line_ending},
	multi::separated_list1,
	sequence::{preceded, separated_pair, terminated, tuple},
};

const COST_A: u64 = 3;
const COST_B: u64 = 1;

const PRIZE_OFFSET: u64 = 10_000_000_000_000;

#[derive(Debug)]
struct ClawMachine {
	a: U64Vec2,
	b: U64Vec2,
	prize: U64Vec2,
}

fn parse_button(input: &str) -> IResult<&str, U64Vec2> {
	preceded(
		{
			if input.chars().nth(7).unwrap() == 'A' {
				tag("Button A: X+")
			} else {
				tag("Button B: X+")
			}
		},
		separated_pair(complete::u64, tag(", Y+"), complete::u64).map(|(x, y)| U64Vec2::new(x, y)),
	)(input)
}

fn parse_prize(input: &str) -> IResult<&str, U64Vec2> {
	preceded(
		tag("Prize: X="),
		separated_pair(complete::u64, tag(", Y="), complete::u64).map(|(x, y)| {
			U64Vec2::new(
				x + if cfg!(test) { 0 } else { PRIZE_OFFSET },
				y + if cfg!(test) { 0 } else { PRIZE_OFFSET },
			)
		}),
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

///
/// use determinants of (prize, b) / determinants of (a, b) to get a
/// solve for b (prize, a) / b
#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	// parse input into usable data
	// x and y for each button, prize location - U64Vec2(x, y)
	let (_, games) = parse(input).map_err(|err| miette!("invalid blocks in input: {}", err))?;

	// token costs of possible paths in vector
	let total_spent: u64 = games
		.iter()
		.filter_map(|game| {
			// Convert to f64
			let (ax, ay) = (game.a.x as f64, game.a.y as f64);
			let (bx, by) = (game.b.x as f64, game.b.y as f64);
			let (px, py) = (game.prize.x as f64, game.prize.y as f64);

			let mat_ab = DMat2::from_cols_array(&[ax, ay, bx, by]);
			let d_ab = mat_ab.determinant();

			// determinant of prize and button b
			let mat_pb = DMat2::from_cols_array(&[px, py, bx, by]);
			let d_pb = mat_pb.determinant();

			// determinant of coordinates a and p
			let mat_ap = DMat2::from_cols_array(&[ax, ay, px, py]);
			let d_ap = mat_ap.determinant();

			let count_btn_a = d_pb / d_ab;
			let count_btn_b = d_ap / d_ab;

			if count_btn_a.trunc() != count_btn_a || count_btn_b.trunc() != count_btn_b {
				None
			} else {
				let max = if cfg!(test) { 100f64 } else { f64::INFINITY };
				if count_btn_a > max || count_btn_b > max {
					None
				} else {
					Some(COST_A * count_btn_a.round() as u64 + COST_B * count_btn_b.round() as u64)
				}
			}
		})
		.sum();

	// minimum tokens to get all possible prizes
	// sum up all possible prizes' token cost
	// Ok(required_tokens.to_string())
	Ok(total_spent.to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	// 	#[test]
	// 	fn f_around_find_out() -> miette::Result<()> {
	// 		let (_, parsed) = parse(
	// 			"Button A: X+94, Y+34
	// Button B: X+22, Y+67
	// Prize: X=8400, Y=5400
	// ",
	// 		)
	// 		.map_err(|e| miette!("fatal"))?;
	// 		// 		let (_, parsed) = parse(
	// 		// 			"Button A: X+26, Y+66
	// 		// Button B: X+67, Y+21
	// 		// Prize: X=12748, Y=12176
	// 		// ",
	// 		// 		)
	// 		// 		.map_err(|e| miette!("fatal"))?;

	// 		// intersections of lines
	// 		// let (ax, ay) = (parsed[0].a.x as f64, parsed[0].a.y as f64);
	// 		// let (bx, by) = (parsed[0].b.x as f64, parsed[0].b.y as f64);
	// 		// let (px, py) = (parsed[0].prize.x as f64, parsed[0].prize.y as f64);
	// 		// // stored in column-major order
	// 		// let mat = DMat2::from_cols_array(&[ax, ay, bx, by]);
	// 		// dbg!(&mat);
	// 		// let d = mat.determinant();
	// 		// dbg!(&d);
	// 		// // px, py, bx, by
	// 		// let mat_ac = DMat2::from_cols_array(&[px, py, bx, by]);
	// 		// dbg!(&d);
	// 		// let d_ac = mat_ac.determinant();
	// 		// dbg!(&d_ac);
	// 		// // ax, ay, px, py
	// 		// let mat_bc = DMat2::from_cols_array(&[ax, ay, px, py]);
	// 		// dbg!(&mat_bc);
	// 		// let d_bc = mat_bc.determinant();
	// 		// dbg!(&d_bc);

	// 		// let x = d_ac / d;
	// 		// let y = d_bc / d;
	// 		// dbg!(x, y);

	// 		// // let token_spent = assert_eq!(459236326669, token_spent);
	// 		// assert!(false);
	// 		Ok(())
	// 	}

	// 	#[test]
	// 	fn test_process() -> miette::Result<()> {
	// 		let input = "Button A: X+94, Y+34
	// Button B: X+22, Y+67
	// Prize: X=10000000008400, Y=10000000005400

	// Button A: X+26, Y+66
	// Button B: X+67, Y+21
	// Prize: X=10000000012748, Y=10000000012176

	// Button A: X+17, Y+86
	// Button B: X+84, Y+37
	// Prize: X=10000000007870, Y=10000000006450

	// Button A: X+69, Y+23
	// Button B: X+27, Y+71
	// Prize: X=10000000018641, Y=10000000010279";
	// 		assert_eq!("480", process(input)?);
	// 		Ok(())
	// 	}
	// 	#[test]
	// 	fn test_first_example() -> miette::Result<()> {
	// 		let input = "Button A: X+94, Y+34
	// Button B: X+22, Y+67
	// Prize: X=8400, Y=5400";
	// 		assert_eq!("280", process(input)?);
	// 		Ok(())
	// 	}
	// 	#[test]
	// 	fn test_second_impossible() -> miette::Result<()> {
	// 		// there are no possible combinations for reaching the prize
	// 		let input = "Button A: X+26, Y+66
	// Button B: X+67, Y+21
	// Prize: X=12748, Y=12176
	// ";
	// 		assert_eq!("4500000000000", process(input)?);
	// 		Ok(())
	// 	}
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
}
