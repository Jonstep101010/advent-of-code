#![allow(unused_imports)]
use std::{
	collections::{HashSet, VecDeque},
	io::Write,
	ops::AddAssign,
};

use grid::{Grid, grid};
use itertools::Itertools;

#[cfg(test)]
fn display_grid(grid: &mut Grid<char>) {
	// clear screen
	print!("\x1B[2J\x1B[1;1H");
	// rotate to get the right orientation for printing
	grid.rotate_left();
	for row in grid.iter_rows() {
		for &c in row {
			print!("{c}");
		}
		println!();
	}
	std::io::stdout().flush().unwrap();
	// reset orientation
	grid.rotate_right();
}

///
/// Example:
/// ```
/// use day_15::part1::print_grid;
/// let input = "#######
/// #...O..
/// #......
/// ";
/// let mut grid = parse_grid(input);
/// assert_eq!(grid.size(), (7, 3));
/// assert_eq!(grid[(0, 2)], '#');
/// assert_eq!(grid[(1, 2)], '#');
/// assert_eq!(grid[(1, 1)], '.');
/// assert_eq!(grid[(4, 1)], 'O');
/// assert_eq!(grid[(3, 1)], '.');
/// assert_eq!(grid[(5, 1)], '.');
/// assert_eq!(grid[(4, 2)], '#');
/// assert_eq!(grid[(0, 1)], '#');
/// assert_eq!(grid[(6, 0)], '.');
/// print_grid(&mut grid);
/// ```
///
/// for each line we want to have have a shelf
/// where the later the line, the lower the shelf will be
/// each line's first character should have an x of 0
#[must_use]
pub fn parse_grid(input: &str) -> Grid<char> {
	let mut new = grid![];
	for row in input.lines().rev() {
		let items = row.chars().collect_vec();
		// map over and insert second same item
		// # -> ##
		// O -> []
		// . -> ..
		// @ -> @.
		// all other characters are left as is
		let items = items
			.iter()
			.flat_map(|&c| match c {
				'#' => vec!['#', '#'],
				'O' => vec!['[', ']'],
				'.' => vec!['.', '.'],
				'@' => vec!['@', '.'],
				other => vec![other],
			})
			.collect_vec();
		new.push_row(items);
	}
	new.transpose();
	new
}

type Direction = Pos;

fn parse_instructions(input: &str) -> Vec<Direction> {
	input
		.replace('\n', "")
		.chars()
		.map(|c| match c {
			'^' => Pos(0, 1),
			'>' => Pos(1, 0),
			'v' => Pos(0, -1),
			'<' => Pos(-1, 0),
			_ => panic!("Invalid direction character"),
		})
		.collect()
}

fn parse(input: &str) -> miette::Result<(Grid<char>, Vec<Direction>)> {
	// preprocess input
	let input = input.chars().collect::<String>();
	let opt_grid_instructions = input.split_once("\n\n");
	if opt_grid_instructions.is_none() {
		return Err(miette::miette!("No instructions/grid found"));
	}
	let (g, ins) = opt_grid_instructions.unwrap();
	let grid = parse_grid(g);
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

#[derive(Debug, Clone)]
struct Warehouse {
	moves: VecDeque<Direction>,
	robot: Pos,
	box_left: HashSet<Pos>,
	box_right: HashSet<Pos>,
	walls: HashSet<Pos>,
	grid: Grid<char>,
	cur_move: Pos,
}

impl Warehouse {
	fn new(
		moves: Vec<Direction>,
		robot: Pos,
		box_left: HashSet<Pos>,
		box_right: HashSet<Pos>,
		walls: HashSet<Pos>,
		grid: Grid<char>,
	) -> Self {
		let cur_move = moves[0];
		Self {
			moves: moves.into(),
			robot,
			box_left,
			box_right,
			walls,
			grid,
			cur_move,
		}
	}
	fn move_robot(&mut self) {
		self.grid[(self.robot.0 as usize, self.robot.1 as usize)] = '.';
		self.robot += self.cur_move;
		self.grid[(self.robot.0 as usize, self.robot.1 as usize)] = '@';
		#[cfg(test)]
		display_grid(&mut self.grid);
	}
	fn move_current_box(&mut self, box_pos: Pos, new_position: Pos) -> Option<Pos> {
		todo!("move boxes");
		// self.boxes.remove(&box_pos);
		// self.boxes.insert(new_position);
		// self.grid[(new_position.0 as usize, new_position.1 as usize)] = 'O';
		// self.grid[(box_pos.0 as usize, box_pos.1 as usize)] = '.';
		// #[cfg(test)]
		// display_grid(&mut self.grid);
		// Some(new_position)
	}
	fn move_box(&mut self, box_pos: Pos) -> Option<Pos> {
		// let new_position = box_pos + self.cur_move;
		// if self.walls.contains(&(box_pos)) {
		// 	None
		// } else if self.grid[(new_position.0 as usize, new_position.1 as usize)] == '.' {
		// 	// the box downstream is free
		// 	return self.move_current_box(box_pos, new_position);
		// } else {
		// 	let next_box = self.move_box(box_pos + self.cur_move);
		// 	// try moving the box downstream
		// 	if next_box.is_some() && next_box.unwrap() == new_position + self.cur_move {
		// 		// move the box
		// 		return self.move_current_box(box_pos, new_position);
		// 	}
		// 	// cannot move the box
		// 	None
		// }
		todo!("move a box");
	}
	///
	/// walk path of possible `moves`
	///
	/// @todo add both horizontal and vertical checks, boxes can only move in specific directions
	pub fn run(&mut self) {
		while !self.moves.is_empty() {
			let mut boxes = self.box_left.union(&self.box_right);
			self.cur_move = self.moves.pop_front().unwrap();
			let next_pos = self.robot + self.cur_move;
			// #[cfg(test)]
			// display_grid(&mut self.grid);
			if self.walls.contains(&next_pos) {
				// skip moves in front of walls
			} else if !self.box_left.contains(&next_pos) && !self.box_right.contains(&next_pos) {
				// we can definitely move: there are no obstacles
				self.move_robot();
			} else if boxes.contains(&next_pos)
				&& !self.walls.contains(&(next_pos + self.cur_move))
				&& !boxes.contains(&(next_pos + self.cur_move))
			{
				// we can move one if a box can move
				self.move_box(next_pos);
				self.move_robot();
			} else if boxes.contains(&(next_pos + self.cur_move))
				&& self.move_box(next_pos).is_some()
			{
				// we can move if we can shift the boxes into free space
				self.move_robot();
			}
		}
	}
	pub fn checksum(&self) -> u64 {
		let mut box_result: u64 = 0;
		let (_, height) = self.grid.size();
		for single_box in &self.box_left {
			box_result += single_box.0 as u64;
			box_result += (height - (single_box.1 + 1) as usize) as u64 * 100;
		}
		box_result
	}
}

fn find_robot(grid: &Grid<char>) -> Pos {
	for (x, mut row) in grid.iter_rows().enumerate() {
		let rowsearch = row.find_position(|&&c| c == '@');
		if rowsearch.is_some() {
			let y = rowsearch.unwrap().0 as i32;
			let robot_pos = Pos(x as i32, y);
			dbg!(&robot_pos);
			return robot_pos;
		}
	}
	unreachable!("there always has to be a robot")
}

fn find_items(grid: &Grid<char>, item: char) -> HashSet<Pos> {
	let mut positions = HashSet::new();
	for (x, row) in grid.iter_rows().enumerate() {
		for (y, &c) in row.enumerate() {
			if c == item {
				positions.insert(Pos(x as i32, y as i32));
			}
		}
	}
	positions
}

const WALL: char = '#';
const BOX_LEFT: char = '[';
const BOX_RIGHT: char = ']';
#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let (grid, movements) = parse(input)?;
	let robot_start = find_robot(&grid);
	let walls = find_items(&grid, WALL);
	let boxes_left = find_items(&grid, BOX_LEFT);
	let boxes_right = find_items(&grid, BOX_RIGHT);
	let mut maze = Warehouse::new(movements, robot_start, boxes_left, boxes_right, walls, grid);
	maze.run();
	let box_checksum = maze.checksum();
	Ok(box_checksum.to_string())
}

#[cfg(test)]
mod tests {
	use std::vec;

	use super::*;

	use rstest::rstest;

	// 	#[rstest]
	// 	#[case(
	// 		"########
	// #..O.O.#
	// ##@.O..#
	// #...O..#
	// #.#.O..#
	// #...O..#
	// #......#
	// ########

	// <^^>>>vv<v>>v<<",
	// 		"2028"
	// 	)]
	// 	// walls: 37, boxes 21
	// 	#[case(
	// 		"##########
	// #..O..O.O#
	// #......O.#
	// #.OO..O.O#
	// #..O@..O.#
	// #O#..O...#
	// #O..O..O.#
	// #.OO.O.OO#
	// #....O...#
	// ##########

	// <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
	// vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
	// ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
	// <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
	// ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
	// ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
	// >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
	// <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
	// ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
	// v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
	// 		"10092"
	// 	)]
	// 	fn test_process(#[case] input: &str, #[case] result: &str) -> miette::Result<()> {
	// 		let output = process(input)?;
	// 		assert_eq!(output, result);
	// 		Ok(())
	// 	}
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
		let grid = parse_grid(input);
		display_grid(&mut grid.clone());
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
		assert_eq!(grid.size(), expected.size());// (20, 10)
		assert_eq!(
			grid.clone().remove_col(5).unwrap(),
			expected.remove_col(4).unwrap()
		);
	}
	#[test]
	fn test_checksum_two() {
		let input = "##########
#.O.O.OOO#
#........#
#OO......#
#OO@.....#
#O#.....O#
#O.....OO#
#O.....OO#
#OO....OO#
##########

<>";
		let (grid, _movements) = parse(input).unwrap();
		let robot_pos = find_robot(&grid);
		assert_eq!(Pos(3, 5), robot_pos);
		let (walls, box_left, box_right) = (
			find_items(&grid, WALL),
			find_items(&grid, BOX_LEFT),
			find_items(&grid, BOX_LEFT),
		);
		let _maze = Warehouse {
			moves: VecDeque::new(),
			robot: robot_pos,
			box_left,
			box_right,
			walls,
			grid,
			cur_move: Pos(0, 0),
		};
		let box_check = _maze.checksum();
		assert_eq!(10092, box_check);
	}
	#[test]
	fn test_find_robot() {
		let input = "####################
##[].......[].[][]##
##[]...........[].##
##[]........[][][]##
##[]......[]....[]##
##..##......[]....##
##..[]............##
##..@......[].[][]##
##......[][]..[]..##
####################
";
		let grid = parse_grid(input);
		let robot_pos = find_robot(&grid);
		assert_eq!(Pos(4, 2), robot_pos);
	}
}
