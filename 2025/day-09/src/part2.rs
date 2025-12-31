use glam::U64Vec2;
use itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let (_, red_tile_pos) = parse(input).expect("parse success");
	let max_areas = red_tile_pos
		.iter()
		.tuple_combinations()
		.map(|(p, q)| (p, q, (p.x.abs_diff(q.x) + 1) * (p.y.abs_diff(q.y) + 1))) // area
		// .map(|(&p, &q)| (p.max(q) - p.min(q) + U64Vec2::ONE).element_product()) // edge lengths axis multiplication (vec.x * vec.y)
		.sorted_by(|a, b| a.2.cmp(&b.2))
		.rev()
		.collect_vec();
	let lines = red_tile_pos
		.iter()
		.circular_tuple_windows()
		.collect::<Vec<(&U64Vec2, &U64Vec2)>>();
	Ok(max_areas
		.iter()
		.find_map(|(p, q, size)| {
			lines
				.iter()
				.all(|(start, end)| {
					// not in bounds
					p.x.max(q.x) <= start.x.min(end.x) // left
					|| p.x.min(q.x) >= start.x.max(end.x)// right
						|| p.y.max(q.y) <= start.y.min(end.y)// above
						|| p.y.min(q.y) >= start.y.max(end.y) //below
				})
				.then_some(size.to_string())
		})
		.unwrap())
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
		assert_eq!("24", process(input)?);
		Ok(())
	}
}
