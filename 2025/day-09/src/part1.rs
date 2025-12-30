use glam::U64Vec2;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let (_, red_tile_pos) = parse(input).expect("parse success");
	let max_area = red_tile_pos
		.iter()
		.tuple_combinations()
		.map(|(p, q)| (p.x.abs_diff(q.x) + 1) * (p.y.abs_diff(q.y) + 1)) // area
		// .map(|(&p, &q)| (p.max(q) - p.min(q) + U64Vec2::ONE).element_product()) // edge lengths axis multiplication (vec.x * vec.y)
		.max()
		.unwrap();
	Ok(max_area.to_string())
}

use itertools::Itertools;
use nom::{IResult, Parser, character::complete::line_ending, multi::separated_list1};
fn parse(input: &str) -> IResult<&str, Vec<U64Vec2>> {
	separated_list1(
		line_ending,
		nom::sequence::separated_pair(
			nom::character::complete::u64,
			nom::bytes::tag(","),
			nom::character::complete::u64,
		)
		.map(|(x, y)| U64Vec2::new(x, y)),
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
