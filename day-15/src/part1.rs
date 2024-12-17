use std::{
	collections::{HashSet, VecDeque},
	ops::AddAssign,
};

use grid::{Grid, grid};
use itertools::Itertools;

pub fn print_grid(grid: &mut Grid<char>) {
	grid.rotate_left();
	println!("grid:");
	for row in grid.iter_rows() {
		for &c in row {
			print!("{}", c);
		}
		println!();
	}
	grid.rotate_right();
}

pub fn dbg_grid(grid: &mut Grid<char>) {
	grid.rotate_left();
	dbg!(&grid);
	grid.rotate_right();
}

///
/// Example:
/// ```
/// use day_15::part1::parse_grid;
/// use day_15::part1::print_grid;
/// use day_15::part1::dbg_grid;
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
// /// grid.rotate_left();// can be used for printing
// /// dbg!(&grid);// prints rotated to the right
// /// grid.rotate_right();// reset
/// print_grid(&mut grid);
/// dbg_grid(&mut grid);
/// ```
pub fn parse_grid(input: &str) -> Grid<char> {
	// for each line we want to have have a shelf
	// where the later the line, the lower the shelf will be
	// each line's first character should have an x of 0
	let mut new = grid![];
	for row in input.lines().rev() {
		let items = row.chars().collect_vec();
		new.push_row(items);
	}
	// for debugging
	// for (lc, row) in input.lines().rev().enumerate() {
	// 	let items = row.chars().collect_vec();
	// 	new.push_row(items);
	// 	for i in 0..row.len() {
	// 		dbg!(new[(lc, i)]);
	// 	}
	// }
	new.transpose();
	print_grid(&mut new);
	new
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
	Up,
	Right,
	Down,
	Left,
}

impl Direction {
	fn to_pos(self) -> Pos {
		match self {
			Direction::Up => Pos(0, 1),
			Direction::Right => Pos(1, 0),
			Direction::Down => Pos(0, -1),
			Direction::Left => Pos(-1, 0),
		}
	}
}

fn parse_instructions(input: &str) -> Vec<Direction> {
	input
		.replace('\n', "")
		.chars()
		.map(|c| match c {
			'^' => Direction::Up,
			'>' => Direction::Right,
			'v' => Direction::Down,
			'<' => Direction::Left,
			_ => panic!("Invalid direction character"),
		})
		.collect()
}

fn parse(input: &str) -> miette::Result<(Grid<char>, Vec<Direction>)> {
	let (g, ins) = input.split_once("\n\n").unwrap();
	let grid = parse_grid(g);
	let instructions = parse_instructions(ins);
	Ok((grid, instructions))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos(i32, i32);
use std::ops::Add;

type Move = Pos;

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

impl Into<(i32, i32)> for Pos {
	fn into(self) -> (i32, i32) {
		(self.0, self.1)
	}
}

#[derive(Debug, Clone)]
struct Warehouse {
	moves: VecDeque<Direction>,
	robot: Pos,
	boxes: HashSet<Pos>,
	walls: HashSet<Pos>,
	size: (usize, usize),
	grid: Grid<char>,
	cur_move: Pos,
}

impl Warehouse {
	fn new(
		moves: Vec<Direction>,
		robot: Pos,
		boxes: HashSet<Pos>,
		walls: HashSet<Pos>,
		size: (usize, usize),
		grid: Grid<char>,
	) -> Self {
		let cur_move = moves[0].clone().to_pos();
		Self {
			moves: moves.into(),
			robot,
			boxes,
			walls,
			size,
			grid,
			cur_move,
		}
	}
	fn move_robot(&mut self) {
		self.grid[(self.robot.0 as usize, self.robot.1 as usize)] = '.';
		self.robot += self.cur_move;
		self.grid[(self.robot.0 as usize, self.robot.1 as usize)] = '#';
	}
	fn move_box(&mut self, box_pos: Pos) {
		// update pos using AddAssign of cur_move
		let mut new_position = box_pos;
		new_position += self.cur_move;
		self.boxes.remove(&box_pos);
		self.boxes.insert(new_position);
		self.grid[(new_position.0 as usize, new_position.1 as usize)] = 'O';
		self.grid[(box_pos.0 as usize, box_pos.1 as usize)] = '.';
	}
	// could return the resulting grid
	pub fn run(&mut self) {
		// check while iterating over movements if a move is possible
		while !self.moves.is_empty() {
			self.cur_move = self.moves.pop_front().unwrap().to_pos();
			let next_pos = self.robot + self.cur_move;
			if self.walls.contains(&next_pos) {
				// do not move
				// skip moves
			} else if !self.boxes.contains(&next_pos) {
				// we can definitely move
				self.move_robot();
			} else {
				// we can move one if a box can move
				if !self.walls.contains(&(next_pos + self.cur_move)) {
					// there is no wall where the box will have to move
					self.move_box(next_pos);
					self.move_robot();
				} else {
					// we cannot move because the box cannot move
				}
			}
		}
	}
	pub fn checksum(&self) -> u64 {
		let mut box_result: u64 = 0;
		let (_, height) = self.size;
		for single_box in &self.boxes {
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
const BOX: char = 'O';
#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let (grid, movements) = parse(input)?;
	// println!("{:?}", grid);
	let robot_start = find_robot(&grid);
	let walls = find_items(&grid, WALL);
	// eprintln!("{:?}", walls);
	let boxes = find_items(&grid, BOX);
	assert_eq!(37, walls.len());
	assert_eq!(21, boxes.len());
	assert!(walls.contains(&Pos(2, 4)));
	assert!(boxes.contains(&Pos(1, 3)));
	assert!(boxes.contains(&Pos(1, 4)));
	let mut maze = Warehouse::new(movements, robot_start, boxes, walls, grid.size(), grid);
	maze.run(); // we might assign this later
	// 100 * distance from top edge + distance from left (x)
	let box_checksum = maze.checksum();
	Ok(box_checksum.to_string())
}

#[cfg(test)]
mod tests {
	use std::{collections::vec_deque, vec};

	use super::*;

	#[test]
	fn test_stuff() {}
	#[test]
	fn test_process() -> miette::Result<()> {
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

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
		assert_eq!("10092", process(input)?);
		Ok(())
	}
	#[test]
	fn test_checksum_one() {
		let input = "#######
#...O..
#......

<>";
		let (grid, _) = parse(input).unwrap();

		assert_eq!(grid.size(), (grid.rows(), grid.cols()));
		assert_eq!((grid.rows(), grid.cols()), (7, 3));
		let (walls, boxes) = (find_items(&grid, WALL), find_items(&grid, BOX));
		let maze = Warehouse {
			moves: VecDeque::new(),
			robot: Pos(0, 0),
			boxes,
			walls,
			size: grid.size(),
			grid,
			cur_move: Pos(0, 0),
		};
		let box_check = maze.checksum();
		assert_eq!(104, box_check);
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
		let (mut grid, _movements) = parse(input).unwrap();
		dbg_grid(&mut grid);
		let robot_pos = find_robot(&grid);
		// Pos(3, 5)
		assert_eq!(Pos(3, 5), robot_pos);
		let (walls, boxes) = (find_items(&grid, WALL), find_items(&grid, BOX));
		let _maze = Warehouse {
			moves: VecDeque::new(),
			robot: robot_pos,
			boxes,
			walls,
			size: grid.size(),
			grid,
			cur_move: Pos(0, 0),
		};
		let box_check = _maze.checksum();
		assert_eq!(10092, box_check);
	}
	#[test]
	fn test_find_robot() {
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
";
		let grid = parse_grid(input);
		let robot_pos = find_robot(&grid);
		assert_eq!(Pos(3, 5), robot_pos);
	}
}
