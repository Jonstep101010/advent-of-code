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

// const directions
const DIRECTIONS: [[i32; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];
fn create_graph_directions(map: &HashMap<(i32, i32), char>) -> UnGraphMap<(i32, i32), ()> {
	// see https://docs.rs/petgraph/latest/petgraph/graphmap/struct.GraphMap.html
	let mut graph: GraphMap</* map */ (i32, i32), (), Undirected> = GraphMap::new();
	for ((x, y), map_c) in map {
		// create node from map
		let node = graph.add_node((*x, *y));
		for [dx, dy] in &DIRECTIONS {
			// create node from direction
			let new_node = (x + dx, y + dy);
			if map.get(&new_node).is_some_and(|c2| map_c == c2) {
				// add edge
				graph.add_edge(node, new_node, ());
			};
		}
	}
	graph
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let map = parse(input);
	dbg!(&map);
	// petgraph? - condense graph
	let graph = create_graph_directions(&map);
	dbg!(&graph);
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
