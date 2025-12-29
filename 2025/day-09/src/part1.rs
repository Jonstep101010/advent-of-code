use glam::IVec2;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let (_, red_tile_pos) = parse(input).expect("parse success");
	dbg!(red_tile_pos);
	// find biggest/smallest x and y
	// calculate size from max difference in each axis
	Ok(0.to_string())
}

use itertools::Itertools;
use miette::IntoDiagnostic;
use nom::{IResult, Parser, character::complete::line_ending, multi::separated_list1};
fn parse(input: &str) -> IResult<&str, Vec<IVec2>> {
	separated_list1(
		line_ending,
		separated_list1(nom::bytes::tag(","), nom::character::complete::i32)
			.map(|v| IVec2::from_slice(&v)),
	)
	.parse(input)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";
		assert_eq!("50", process(input)?);
		Ok(())
	}
}
