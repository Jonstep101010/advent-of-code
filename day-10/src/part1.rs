#![warn(clippy::pedantic)]

use std::collections::HashMap;

fn find_path_rec(grid: &mut Vec<Vec<u32>>, position: (usize, usize), altitude: u32) -> u32 {
	let (x, y) = position;
	if x >= grid.len() || y >= grid[0].len() {
		return 0;
	}
	if altitude != grid[x][y] {
		return 0;
	}
	if grid[x][y] == 9 {
		grid[x][y] = 0;
		return 1;
	}
	let mut peak_count = 0;
	if x > 0 {
		peak_count += find_path_rec(grid, (x - 1, y), altitude + 1);
	}
	if x < grid.len() - 1 {
		peak_count += find_path_rec(grid, (x + 1, y), altitude + 1);
	}
	if y > 0 {
		peak_count += find_path_rec(grid, (x, y - 1), altitude + 1);
	}
	if y < grid[0].len() - 1 {
		peak_count += find_path_rec(grid, (x, y + 1), altitude + 1);
	}
	peak_count
}

use itertools::Itertools;
pub fn process(input: &str) -> miette::Result<String> {
	let mut rgrid = vec![];
	for line in input.lines() {
		rgrid.push(
			line.chars()
				.map(|char| char.to_digit(10).unwrap())
				.collect_vec(),
		);
	}
	let mut rtrailheads = HashMap::new();
	for (i, grid) in rgrid.iter().enumerate() {
		for (j, grid) in grid.iter().enumerate() {
			let x = *grid;
			if x == 0 {
				rtrailheads.insert((i, j), true);
			}
		}
	}
	let score = {
		rtrailheads
			.keys()
			.into_iter()
			.map(|th| {
				let mut grid = rgrid.clone();
				find_path_rec(&mut grid, *th, 0)
			})
			.sum::<u32>()
	};
	Ok(score.to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
		assert_eq!("36", process(input)?);
		Ok(())
	}
}
