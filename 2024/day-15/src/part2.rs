#![allow(unused_imports)]
use std::{
	collections::{HashSet, VecDeque},
	io::Write,
	ops::AddAssign,
};

use grid::{Grid, grid};
use itertools::Itertools;

#[cfg(not(test))]
fn display_grid(grid: &Grid<char>) {
	// clear screen
	print!("\x1B[2J\x1B[1;1H");
	// rotate to get the right orientation for printing
	let mut grid = grid.clone();
	grid.rotate_left();
	for row in grid.iter_rows() {
		for &c in row {
			print!("{c}");
		}
		println!();
	}
	std::io::stdout().flush().unwrap();
	std::thread::sleep(std::time::Duration::from_millis(20))
}

#[must_use]
pub fn parse_grid(input: &str) -> Grid<char> {
	let mut new = grid![];
	for row in input.lines().rev() {
		let items = row.trim().chars().collect_vec();
		if !items.is_empty() {
			new.push_row(items);
		}
	}
	new.transpose();
	new
}

type Direction = Pos;

fn parse_instructions(input: &str) -> Vec<Direction> {
	input
		.replace('\n', "")
		.chars()
		.filter(|c| !c.is_whitespace())
		.map(|c| match c {
			'^' => Pos(0, 1),
			'>' => Pos(1, 0),
			'v' => Pos(0, -1),
			'<' => Pos(-1, 0),
			c => panic!("Invalid direction character: '{c}'"),
		})
		.collect()
}

fn expand_input(input: &str) -> String {
	input
		.lines()
		.map(|line| {
			let mut new_line = String::new();
			for c in line.chars() {
				match c {
					'#' => new_line.push_str("##"),
					'O' => new_line.push_str("[]"),
					'.' => new_line.push_str(".."),
					'@' => new_line.push_str("@."),
					_ => new_line.push(c),
				}
			}
			new_line
		})
		.join("\n")
}

fn parse(input: &str) -> miette::Result<(Grid<char>, Vec<Direction>)> {
	let opt_grid_instructions = input.split_once("\n\n");
	if opt_grid_instructions.is_none() {
		return Err(miette::miette!("No instructions/grid found"));
	}
	let (g, ins) = opt_grid_instructions.unwrap();
	let expanded_g = expand_input(g);
	let grid = parse_grid(&expanded_g);
	let instructions = parse_instructions(ins);
	Ok((grid, instructions))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos(i32, i32);
use std::ops::Add;

impl Add for Pos {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Pos(self.0 + other.0, self.1 + other.1)
	}
}

impl AddAssign for Pos {
	fn add_assign(&mut self, rhs: Self) {
		self.0 += rhs.0;
		self.1 += rhs.1;
	}
}

impl From<Pos> for (i32, i32) {
	fn from(val: Pos) -> Self {
		(val.0, val.1)
	}
}

impl Pos {
	fn to_indices(&self) -> (usize, usize) {
		(self.0 as usize, self.1 as usize)
	}
}

fn checksum_grid(grid: &Grid<char>) -> u64 {
	let mut result: u64 = 0;
	let (_, height) = grid.size();
	for (x, row) in grid.iter_rows().enumerate() {
		for (y, &c) in row.enumerate() {
			if c == '[' {
				let dist_top = height - 1 - y;
				let dist_left = x;
				result += (100 * dist_top + dist_left) as u64;
			}
		}
	}
	result
}

#[derive(Debug, Clone)]
struct Warehouse {
	moves: VecDeque<Direction>,
	robot: Pos,
	grid: Grid<char>,
}

impl Warehouse {
	fn new(moves: Vec<Direction>, robot: Pos, grid: Grid<char>) -> Self {
		Self {
			moves: moves.into(),
			robot,
			grid,
		}
	}

	fn get_char(&self, pos: Pos) -> char {
		self.grid[pos.to_indices()]
	}

	fn set_char(&mut self, pos: Pos, c: char) {
		let (x, y) = pos.to_indices();
		self.grid[(x, y)] = c;
	}

	fn collect_boxes(&self, start_box_l: Pos, dir: Pos) -> Option<HashSet<Pos>> {
		let mut to_visit = vec![start_box_l];
		let mut visited = HashSet::new();
		visited.insert(start_box_l);

		let mut idx = 0;
		while idx < to_visit.len() {
			let curr_l = to_visit[idx];
			idx += 1;
			let curr_r = curr_l + Pos(1, 0);

			// Determine next positions for this box
			let next_l = curr_l + dir;
			let next_r = curr_r + dir;

			// Check collisions
			// We need to check the cells that are NOT part of the current box
			// If moving Right (1, 0): check next_r (which is curr_r + 1)
			// If moving Left (-1, 0): check next_l (which is curr_l - 1)
			// If moving Up/Down: check next_l and next_r

			let mut check_positions = Vec::new();
			if dir == Pos(1, 0) {
				check_positions.push(next_r);
			} else if dir == Pos(-1, 0) {
				check_positions.push(next_l);
			} else {
				check_positions.push(next_l);
				check_positions.push(next_r);
			}

			for pos in check_positions {
				let c = self.get_char(pos);
				if c == '#' {
					return None;
				}
				if c == '[' || c == ']' {
					// Found another box
					let neighbor_l = if c == '[' { pos } else { pos + Pos(-1, 0) };
					if !visited.contains(&neighbor_l) {
						visited.insert(neighbor_l);
						to_visit.push(neighbor_l);
					}
				}
			}
		}
		Some(visited)
	}

	fn move_robot(&mut self) {
		if let Some(dir) = self.moves.pop_front() {
			let next_pos = self.robot + dir;
			let c = self.get_char(next_pos);

			if c == '#' {
				// Wall, do nothing
			} else if c == '.' {
				// Empty, move
				self.set_char(self.robot, '.');
				self.robot = next_pos;
				self.set_char(self.robot, '@');
			} else if c == '[' || c == ']' {
				// Box
				let box_l = if c == '[' {
					next_pos
				} else {
					next_pos + Pos(-1, 0)
				};
				if let Some(boxes) = self.collect_boxes(box_l, dir) {
					// Move all boxes
					// 1. Clear all boxes
					for &b_l in &boxes {
						let b_r = b_l + Pos(1, 0);
						self.set_char(b_l, '.');
						self.set_char(b_r, '.');
					}
					// 2. Place all boxes in new positions
					for &b_l in &boxes {
						let new_l = b_l + dir;
						let new_r = new_l + Pos(1, 0);
						self.set_char(new_l, '[');
						self.set_char(new_r, ']');
					}
					// 3. Move robot
					self.set_char(self.robot, '.');
					self.robot = next_pos;
					self.set_char(self.robot, '@');
				}
			}
		}
		#[cfg(not(test))]
		display_grid(&self.grid);
	}

	pub fn run(&mut self) {
		while !self.moves.is_empty() {
			self.move_robot();
			// #[cfg(test)]
			// display_grid(&self.grid);
		}
	}

	pub fn checksum(&self) -> u64 {
		checksum_grid(&self.grid)
	}
}

fn find_robot(grid: &Grid<char>) -> Pos {
	for (x, col) in grid.iter_rows().enumerate() {
		for (y, &c) in col.enumerate() {
			if c == '@' {
				return Pos(x as i32, y as i32);
			}
		}
	}
	unreachable!("there always has to be a robot")
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let (grid, movements) = parse(input)?;
	let robot_start = find_robot(&grid);
	let mut maze = Warehouse::new(movements, robot_start, grid);
	maze.run();
	Ok(maze.checksum().to_string())
}

#[cfg(test)]
mod tests {
	use std::vec;

	use super::*;

	use rstest::rstest;

	#[rstest]
	// walls: 37, boxes 21
	#[case(
		"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
		"9021"
	)]
	fn test_process(#[case] input: &str, #[case] result: &str) -> miette::Result<()> {
		let output = process(input)?;
		assert_eq!(output, result);
		Ok(())
	}
	#[test]
	fn test_parse_wide_grid() {
		let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########
";
		let expanded = expand_input(input);
		let grid = parse_grid(&expanded);
		// grows down
		let mut expected: Grid<char> = grid![];
		expected.push_col("####################".chars().collect_vec());
		expected.push_col("##....[]....[]..[]##".chars().collect_vec());
		expected.push_col("##............[]..##".chars().collect_vec());
		expected.push_col("##..[][]....[]..[]##".chars().collect_vec());
		expected.push_col("##....[]@.....[]..##".chars().collect_vec());
		expected.push_col("##[]##....[]......##".chars().collect_vec());
		expected.push_col("##[]....[]....[]..##".chars().collect_vec());
		expected.push_col("##..[][]..[]..[][]##".chars().collect_vec());
		expected.push_col("##........[]......##".chars().collect_vec());
		expected.push_col("####################".chars().collect_vec());
		let expected_player_row = vec![
			'#', '#', '.', '.', '.', '.', '[', ']', '@', '.', '.', '.', '.', '.', '[', ']', '.',
			'.', '#', '#',
		];
		assert_eq!(grid.clone().remove_col(5).unwrap(), expected_player_row);
		assert_eq!(grid.size(), expected.size()); // (20, 10)
		assert_eq!(
			grid.clone().remove_col(5).unwrap(),
			expected.remove_col(4).unwrap()
		);
	}
	#[test]
	fn test_checksum_one() {
		let mut expected_moves_done: Grid<char> = grid![];
		expected_moves_done.push_col("##########".chars().collect_vec());
		expected_moves_done.push_col("##...[]...".chars().collect_vec());
		expected_moves_done.push_col("##........".chars().collect_vec());
		let box_check = checksum_grid(&expected_moves_done);
		assert_eq!(105, box_check);
	}
	#[test]
	fn test_checksum_two() {
		let input = "####################
##[].......[].[][]##
##[]...........[].##
##[]........[][][]##
##[]......[]....[]##
##..##......[]....##
##..[]............##
##..@......[].[][]##
##......[][]..[]..##
####################";
		let expected_moves_done = parse_grid(input);
		let box_check = checksum_grid(&expected_moves_done);
		assert_eq!(9021, box_check);
	}
	#[test]
	fn test_find_robot() {
		let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########
";
		let grid = parse_grid(input);
		let robot_pos = find_robot(&grid);
		assert_eq!(Pos(2, 5), robot_pos);
	}
}
