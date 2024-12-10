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
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::missing_errors_doc)]
pub fn process(input: &str) -> miette::Result<String> {
	let rgrid: Vec<Vec<u32>> = input
		.lines()
		.map(|line| {
			line.chars()
				.map(|char| char.to_digit(10).expect("grid cannot contain non-digits"))
				.collect_vec()
		})
		.collect_vec();
	let rtrailheads: HashMap<(usize, usize), bool> = rgrid
		.iter()
		.enumerate()
		.flat_map(|(i, grid)| {
			grid.iter().enumerate().filter_map(
				move |(j, &x)| {
					if x == 0 { Some(((i, j), true)) } else { None }
				},
			)
		})
		.collect();
	let score = {
		rtrailheads
			.keys()
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
