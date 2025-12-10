#![warn(clippy::pedantic)]

use petgraph::prelude::*;
use petgraph::visit::IntoNodeReferences;
use std::collections::HashMap;
use std::io::Write;

#[allow(clippy::cast_possible_wrap)]
#[allow(clippy::cast_possible_truncation)]
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
///
/// for every node in the map, create a node in the graph
/// for every direction from a node, create edges in the graph
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

#[allow(dead_code)]
fn write_graph_file(
	graph: &Graph<Vec<(i32, i32)>, (), Undirected, NodeIndex>,
) -> miette::Result<()> {
	std::fs::File::create("example_graph.dot")
		.and_then(|mut file| writeln!(file, "{:?}", petgraph::dot::Dot::with_config(&graph, &[])))
		.map_err(|e| miette::miette!(e.to_string()))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let map = parse(input);
	// could directly create graph from input, but this way we can debug the map
	let graph = create_graph_directions(&map);
	// every plant is a component of Node, part of a garden, collect gardens in graph
	let graph_of_gardens_with_plants: Graph<Vec<(i32, i32)>, (), Undirected, NodeIndex> =
		petgraph::algo::condensation(graph.clone().into_graph::<NodeIndex>(), false);
	#[cfg(test)]
	{
		write_graph_file(&graph_of_gardens_with_plants)?;
	}
	let price_all_areas: usize = graph_of_gardens_with_plants
		.node_references() // for all gardens
		.map(|(_, garden)| {
			let area = garden.len();
			let perimeter = garden
				.iter()
				.map(|&(x, y)| {
					// for each plant in garden, count the number of neighbors
					// subtract from 4 to get the number of edges
					4 - graph.neighbors((x, y)).count()
				})
				.sum::<usize>();
			// area price = perimeter * amount inside perimeter
			perimeter * area
		})
		// .count()// number of gardens
		.sum();
	Ok(price_all_areas.to_string())
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
