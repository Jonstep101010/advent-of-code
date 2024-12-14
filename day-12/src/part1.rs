#![warn(clippy::pedantic)]

use petgraph::prelude::*;
use std::collections::HashMap;

fn parse(input: &str) -> HashMap<(i32, i32), char> {
	input
		.lines()
		.map(|line| line.chars())
		.enumerate()
		.flat_map(|(y, row)| row.enumerate().map(move |(x, c)| ((x as i32, y as i32), c)))
		.collect()
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let map = parse(input);
	dbg!(map);
	// petgraph?
	// const directions
	// area price = perimeter * amount inside perimeter
	// sum of all area prices
	let mut total_price = 0;
	Ok(total_price.to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
		assert_eq!("1930", process(input)?);
		Ok(())
	}
}
