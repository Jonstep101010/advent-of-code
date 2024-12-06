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

const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

// Helper function
fn red(text: &str) -> String {
	format!("{}{}{}", RED, text, RESET)
}

#[cfg(not(test))]
fn print_grid(grid: &Vec<Vec<char>>) {
	print!("\x1B[2J\x1B[1;1H");
	for row in grid {
		eprintln!("{}", row.iter().collect::<String>());
	}
	std::io::Write::flush(&mut std::io::stdout()).unwrap();
}

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
// traverse from part1
fn get_path(grid: &[Vec<char>]) -> ((usize, usize), HashSet<(usize, usize, usize)>) {
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

		if y >= grid.len() || x >= grid[0].len() {
			break;
		}

		let next = grid[y][x];
		if next == '#' {
			// let mut next_directions = POSSIBLE_DIRECTIONS.clone();
			match turns {
				3 => {
					turns = 0;
				}
				_ => {
					turns += 1;
				}
			}
			traversed[prev_y][prev_x] = '+';
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
	#[cfg(test)]
	{
		for (turns, y, x) in &unique_positions {
			eprintln!("{}", red(&format!("{} {} {}", turns, y, x)));
		}
		eprintln!("wall positions");
		for (y, x) in &wall_positions {
			eprintln!("{}", red(&format!("  {} {}", y, x)));
		}
		assert_eq!(41, unique_positions.len());
	}
	print_grid(&traversed);
	(start_pos, unique_positions)
}

// assert_eq!(path.get(&(0, start_pos.0, start_pos.1)).unwrap(), &(0 as usize, 6 as usize, 4 as usize));
fn traverse(grid: &[Vec<char>]) -> usize {
	let (start_pos, path) = get_path(grid);
	let mut traversed = grid.to_owned();

	// (dir/turns, y, x)
	let mut already_passed: HashSet<(/* turns */usize, usize, usize)> = HashSet::new();
	
	let mut turns = 0;
	let mut cur_direction = POSSIBLE_DIRECTIONS[0];
	let mut cur_position = start_pos;
	loop {
		let (cur_y, cur_x) = cur_position;
		let (y, x) = (
			(cur_y as i32 + cur_direction.0) as usize,
			(cur_x as i32 + cur_direction.1) as usize,
		);

		if y >= grid.len() || x >= grid[0].len() {
			break;
		}

		let next = grid[y][x];
		match next {
			'#' => {
				// let mut next_directions = POSSIBLE_DIRECTIONS.clone();
				match turns {
					3 => {
						turns = 0;
					}
					_ => {
						turns += 1;
					}
				}
				cur_direction = POSSIBLE_DIRECTIONS[turns];
				let (ny, nx) = (
					(cur_y as i32 + cur_direction.0) as usize,
					(cur_x as i32 + cur_direction.1) as usize,
				);
				if ny >= grid.len() || nx >= grid[0].len() {
					break;
				}
				if traversed[cur_y][cur_x] == '|' || traversed[cur_y][cur_x] == '-' {
					traversed[cur_y][cur_x] = '+';
				}
			}
			_ => {
				assert_ne!(traversed[y][x], 'X');
				if traversed[y][x] != '|' && traversed[y][x] != '-' && traversed[y][x] != '^' && traversed[y][x] != '+' {
					unique_positions += 1;
				} else {
					already_passed.insert((turns, y, x));
				}
				if traversed[y][x] != '^' {
					traversed[y][x] = match cur_direction {
						(_, 0) => '|',
						(0, _) => '-',
						_ => unreachable!(),
					};
				}

				cur_position = (y, x);
			}
		}
		#[cfg(test)]
		{
			print_grid(&traversed);
		}
	}
	#[cfg(test)]
	{
		print_grid(&traversed);
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
		assert_eq!("6", process(input)?);
		Ok(())
	}
}
