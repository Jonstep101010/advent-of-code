fn find_start(grid: &Vec<Vec<char>>) -> (usize, usize) {
	for (y, row) in grid.iter().enumerate() {
		for (x, &ch) in row.iter().enumerate() {
			if ch == '^' {
				return (y, x);
			}
		}
	}
	panic!("No starting position found");
}

#[cfg(test)]
fn print_grid(grid: &Vec<Vec<char>>) {
	eprintln!();
	for row in grid {
		eprintln!("{}", row.iter().collect::<String>());
	}
}

fn traverse(grid: &Vec<Vec<char>>) -> usize {
	let mut traversed = grid.clone();
	let mut unique_positions = 1;

	// Start at ^ position (6,4)
	let mut prev_position = find_start(grid);

	let possible_directions = vec![
		(-1, 0), // up
		(0, 1),  // right
		(1, 0),  // down
		(0, -1), // left
	];
	let mut turns = 0;

	let mut cur_direction = possible_directions[turns]; // Start going up
	// traversed[prev_position.0][prev_position.1] = 'X';

	let mut cur_direction = possible_directions[0];

	loop {
		let (prev_y, prev_x) = prev_position;
		let (y, x) = (
			(prev_y as i32 + cur_direction.0) as usize,
			(prev_x as i32 + cur_direction.1) as usize,
		);

		if y >= grid.len() || x >= grid[0].len() {
			break;
		}

		let mut next = grid[y][x];
		if next == '#' {
			// let mut next_directions = possible_directions.clone();
			match turns {
				3 => {
					turns = 0;
				}
				_ => {
					turns += 1;
				}
			}
			cur_direction = possible_directions[turns];
			let (ny, nx) = (
				(prev_y as i32 + cur_direction.0) as usize,
				(prev_x as i32 + cur_direction.1) as usize,
			);
			if ny >= grid.len() || nx >= grid[0].len() {
				break;
			}
		} else {
			if traversed[y][x] != 'X' && traversed[y][x] != '^' {
				unique_positions += 1;
			}
			traversed[y][x] = 'X';
			prev_position = (y, x);
		}
		#[cfg(test)]
		{
			print_grid(&traversed);
		}
	}
	#[cfg(test)]
	{
		print_grid(&traversed);
		// compare with end.txt
		let end = include_str!("end.txt");
		let end_grid: Vec<Vec<char>> = end.lines().map(|line| line.chars().collect()).collect();
		assert_eq!(end_grid, traversed);
	}
	unique_positions
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
	// assert_eq!((6, 4), find_start(&grid));
	Ok(traverse(&grid).to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
		assert_eq!("41", process(input)?);
		Ok(())
	}
}
