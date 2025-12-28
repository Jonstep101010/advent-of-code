use glam::IVec3;
use petgraph::unionfind::UnionFind;
#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
	let (_, pos) = parse(_input).expect("correct parse");
	let mut connections = UnionFind::new(pos.len());
	for ((idx_a, vec_a), (idx_b, vec_b), _) in pos
		.iter()
		.enumerate()
		.tuple_combinations()
		.map(|((idx_a, vec_a), (idx_b, vec_b))| {
			(
				(idx_a, vec_a),
				(idx_b, vec_b),
				vec_a.as_vec3().distance(vec_b.as_vec3()),
			)
		}) // dist
		.sorted_by(|(_, _, dist_a), (_, _, dist_b)| dist_a.partial_cmp(dist_b).unwrap())
	// not by_key as no float ord
	{
		// check if unified (merged), check if all in pos in connections, return prod(x positions)
		if connections.union(idx_a, idx_b)
			&& (0..pos.len())
				.tuple_windows()
				.all(|(a, b)| connections.equiv(a, b))
		{
			return Ok((vec_a.x * vec_b.x).to_string());
		}
	}
	unreachable!()
}

use itertools::Itertools;
use nom::{IResult, Parser, character::complete::line_ending, multi::separated_list1};
fn parse(input: &str) -> IResult<&str, Vec<IVec3>> {
	separated_list1(
		line_ending,
		separated_list1(nom::bytes::tag(","), nom::character::complete::i32)
			.map(|v| IVec3::from_slice(&v)),
	)
	.parse(input)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";
		// fully connected, cannot reach 40 (unless input twice as long)
		assert_eq!("25272", process(input)?);
		Ok(())
	}
}
