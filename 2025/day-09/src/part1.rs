use glam::IVec2;

fn square(p: &IVec2, q: &IVec2) -> u64 {
	let tmp = u64::from((p.x.abs_diff(q.x) + 1) * (p.y.abs_diff(q.y) + 1));
	dbg!(tmp)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let (_, red_tile_pos) = parse(input).expect("parse success");
	// dbg!(&red_tile_pos);
	// find biggest/smallest x and y
	// calculate size from max difference in each axis
	// biggest is in between 2/5 and 11/1
	let max_area = red_tile_pos
		.iter()
		.tuple_combinations()
		.map(|(a, b)| {
			square(a, b)
			// dbg!(a, b);
			// some max calc
		})
		.max()
		.unwrap();
	Ok(max_area.to_string())
}

use itertools::Itertools;
use nom::{IResult, Parser, character::complete::line_ending, multi::separated_list1};
fn parse(input: &str) -> IResult<&str, Vec<IVec2>> {
	separated_list1(
		line_ending,
		nom::sequence::separated_pair(
			nom::character::complete::i32,
			nom::bytes::tag(","),
			nom::character::complete::i32,
		)
		.map(|(x, y)| IVec2::new(x, y)),
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
