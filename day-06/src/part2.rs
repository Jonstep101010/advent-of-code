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

fn traverse(grid: &[Vec<char>]) -> usize {
	let mut traversed = grid.to_owned();
	let mut unique_positions = 1;
	let mut already_passed = 0;

	// Start at ^ position (6,4)
	let start_pos = find_start(grid);
	let mut prev_position = start_pos;

	let possible_directions = [
		(-1, 0), // up
		(0, 1),  // right
		(1, 0),  // down
		(0, -1), // left
	];
	let mut turns = 0;

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

		let next = grid[y][x];
		match next {
			'#' => {
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
				if traversed[prev_y][prev_x] == '|' || traversed[prev_y][prev_x] == '-' {
					traversed[prev_y][prev_x] = '+';
				}
			}
			_ => {
				assert_ne!(traversed[y][x], 'X');
				if traversed[y][x] != '|' && traversed[y][x] != '-' && traversed[y][x] != '^' {
					unique_positions += 1;
				} else {
					already_passed += 1;
				}
				if traversed[y][x] != '^' {
					traversed[y][x] = match cur_direction {
						(_, 0) => '|',
						(0, _) => '-',
						_ => unreachable!(),
					};
				} else {
				}

				prev_position = (y, x);
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
