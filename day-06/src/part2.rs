use std::collections::HashSet;

#[allow(dead_code)]
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";
#[allow(dead_code)]
// Helper function
fn red(text: &str) -> String {
	format!("{}{}{}", RED, text, RESET)
}

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

const MAX_ITERATIONS: usize = 10000;

fn walk_path(
	grid: &[Vec<char>],
) -> (
	(usize, usize),
	HashSet<(usize, usize, usize)>,
	Vec<Vec<char>>,
	bool,
) {
	let mut traversed = grid.to_owned();
	let mut unique_positions: HashSet<(/* turns */ usize, usize, usize)> = HashSet::new();

	// Start at ^ position (6,4)
	let start_pos = find_start(grid);
	let mut prev_position = start_pos;
	let mut visited_states: HashSet<(usize, usize, usize)> = HashSet::new(); // Track (turns, y, x)
	unique_positions.insert((0, prev_position.0, prev_position.1));
	let mut wall_positions: HashSet<(usize, usize)> = HashSet::new();
	let mut turns = 0;
	let mut is_loop = false;

	let mut cur_direction = POSSIBLE_DIRECTIONS[0];
	let mut iterations = 0;

	loop {
		iterations += 1;
		if iterations > MAX_ITERATIONS {
			// escape hatch
			is_loop = true;
			break;
		}

		let (prev_y, prev_x) = prev_position;
		let (y, x) = (
			(prev_y as i32 + cur_direction.0) as usize,
			(prev_x as i32 + cur_direction.1) as usize,
		);
		// Check for loop
		let state = (turns, y, x);
		if y >= grid.len() || x >= grid[0].len() {
			break;
		}
		if !visited_states.insert(state) {
			// We've seen this state before - we're in a loop
			is_loop = true;
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
	(start_pos, visited_states, traversed, is_loop)
}

///
/// check for the path taken, what unique obstacles would cause a loop
fn traverse(grid: &[Vec<char>]) -> usize {
	let (start_pos, unique_positions, _, _) = walk_path(grid);
	// we map the obstacles on the unique positions
	// we need to reset it to the start position on each iteration
	let mut obstacles_cause_loop: HashSet<(usize, usize)> = HashSet::new();
	// we will brute force the path: check for all unique positions if placing an obstacle would cause a loop

	for (turns, y, x) in &unique_positions {
		let mut loop_check_items: HashSet<(/* turns */ usize, usize, usize)> = HashSet::new();
		loop_check_items.insert((*turns, *y, *x));
		let (ny, nx) = (*y, *x);
		// check if placing an obstacle would cause a loop by checking if the path is still traversable
		let mut grid_with_obstacle: Vec<Vec<char>> = grid.to_vec();
		if (ny, nx) != start_pos {
			grid_with_obstacle[ny][nx] = '0';
			// check if it runs a certain amount of time (it might never end)
			let (_, _, _, is_loop) = walk_path(&grid_with_obstacle);
			if is_loop {
				// eprintln!("obstacle causes loop");
				// print_grid(&grid_with_obstacle);
				obstacles_cause_loop.insert((ny, nx));
			}
		}
	}
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
	fn test_walk_path_contains_loop() {
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
		assert_eq!(true, walk_path(&grid).3);
	}
}
