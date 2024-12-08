use std::collections::HashSet;

fn find_start(grid: &[Vec<char>]) -> (usize, usize) {
	for (y, row) in grid.iter().enumerate() {
		for (x, &ch) in row.iter().enumerate() {
			if ch == '^' {
				return (y, x);
			}
		}
	}
	panic!("No starting position found");
}

#[allow(dead_code)]
#[cfg(not(test))]
fn print_grid(grid: &Vec<Vec<char>>) {
	print!("\x1B[2J\x1B[1;1H");
	for row in grid {
		eprintln!("{}", row.iter().collect::<String>());
	}
	std::io::Write::flush(&mut std::io::stdout()).unwrap();
}

#[allow(dead_code)]
#[cfg(test)]
fn print_grid(grid: &Vec<Vec<char>>) {
	eprintln!();
	for row in grid {
		eprintln!("{}", row.iter().collect::<String>());
	}
}

const POSSIBLE_DIRECTIONS: [(i32, i32); 4] = [
	(-1, 0), // up
	(0, 1),  // right
	(1, 0),  // down
	(0, -1), // left
];

fn check_infinite_loop(start_pos: (usize, usize), grid: &[Vec<char>]) -> bool {
	let mut visited_states: HashSet<((usize, usize), usize)> = HashSet::new();
	let mut prev_position = start_pos;
	let mut turns = 0;
	let mut cur_direction = POSSIBLE_DIRECTIONS[turns];

	loop {
		let (prev_y, prev_x) = prev_position;
		let (y, x) = (
			(prev_y as i32 + cur_direction.0) as usize,
			(prev_x as i32 + cur_direction.1) as usize,
		);

		if y >= grid.len() || x >= grid[0].len() {
			return false;
		}

		let state = ((y, x), turns);
		if !visited_states.insert(state) {
			return true;
		}

		let next = grid[y][x];
		if next == '#' || next == '0' {
			turns = if turns == 3 { 0 } else { turns + 1 };
			cur_direction = POSSIBLE_DIRECTIONS[turns];

			// Check next position validity
			let (ny, nx) = (
				(prev_y as i32 + cur_direction.0) as usize,
				(prev_x as i32 + cur_direction.1) as usize,
			);
			if ny >= grid.len() || nx >= grid[0].len() {
				return false;
			}
		} else {
			prev_position = (y, x);
		}
	}
}

#[allow(clippy::type_complexity)]
fn walk_path(grid: &[Vec<char>]) -> ((usize, usize), HashSet<(usize, usize, usize)>) {
	let mut traversed = grid.to_owned();
	let mut unique_positions: HashSet<(/* turns */ usize, usize, usize)> = HashSet::new();

	// Start at ^ position (6,4)
	let start_pos = find_start(grid);
	let mut prev_position = start_pos;
	unique_positions.insert((0, prev_position.0, prev_position.1));
	let mut wall_positions: HashSet<(usize, usize)> = HashSet::new();
	let mut turns = 0;

	let mut cur_direction = POSSIBLE_DIRECTIONS[0];
	loop {
		let (prev_y, prev_x) = prev_position;
		let (y, x) = (
			(prev_y as i32 + cur_direction.0) as usize,
			(prev_x as i32 + cur_direction.1) as usize,
		);
		// Check for loop
		if y >= grid.len() || x >= grid[0].len() {
			break;
		}
		let next = grid[y][x];
		if next == '#' || next == '0' {
			// let mut next_directions = POSSIBLE_DIRECTIONS.clone();
			match turns {
				3 => {
					turns = 0;
				}
				_ => {
					turns += 1;
				}
			}
			if (prev_y, prev_x) != start_pos {
				traversed[prev_y][prev_x] = '+';
			}
			wall_positions.insert((y, x));
			cur_direction = POSSIBLE_DIRECTIONS[turns];
			let (ny, nx) = (
				(prev_y as i32 + cur_direction.0) as usize,
				(prev_x as i32 + cur_direction.1) as usize,
			);
			if ny >= grid.len() || nx >= grid[0].len() {
				break;
			}
		} else {
			if traversed[y][x] != '-'
				&& traversed[y][x] != '|'
				&& traversed[y][x] != '^'
				&& traversed[y][x] != '+'
			{
				unique_positions.insert((turns, y, x));
			}
			match turns {
				0 | 2 => {
					if traversed[y][x] == '.' {
						traversed[y][x] = '|';
					}
				}
				1 | 3 => {
					if traversed[y][x] == '.' {
						traversed[y][x] = '-';
					}
				}
				_ => {
					panic!("Invalid turn");
				}
			}
			prev_position = (y, x);
		}
	}
	(start_pos, unique_positions)
}

///
/// check for the path taken, what unique obstacles would cause a loop
fn traverse(grid: &[Vec<char>]) -> usize {
	let (start_pos, unique_positions) = walk_path(grid);
	// we map the obstacles on the unique positions
	// we need to reset it to the start position on each iteration, passing start_pos to check.
	let obstacles_cause_loop: HashSet<_> = unique_positions
		.into_iter()
		.filter(|(_, y, x)| ((*y, *x) != start_pos))
		.filter_map(|(_, y, x)| {
			let mut grid_with_obstacle = grid.to_vec();
			grid_with_obstacle[y][x] = '0';
			check_infinite_loop(start_pos, &grid_with_obstacle).then_some((y, x))
		})
		.collect();
	obstacles_cause_loop.len()
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
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
		assert_eq!("6", process(input)?);
		Ok(())
	}
	#[test]
	fn test_check_infinite_loop() {
		let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#.0^.....
........#.
#.........
......#...";
		let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
		let start_pos = (6, 4);
		assert_eq!(true, check_infinite_loop(start_pos, &grid));
	}
}
