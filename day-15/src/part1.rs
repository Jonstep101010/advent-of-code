use std::collections::HashSet;

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
	for (lc, row) in input.lines().rev().enumerate() {
		let items = row.chars().collect_vec();
		new.push_row(items);
		eprintln!("lc: {lc}");
		for i in 0..row.len() {
			dbg!(new[(lc, i)]);
		}
	}
	dbg!(&new);
	new.transpose();
	dbg!(&new);
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
	fn to_offset(self) -> (i32, i32) {
		match self {
			Direction::Up => (0, 1),
			Direction::Right => (1, 0),
			Direction::Down => (0, -1),
			Direction::Left => (-1, 0),
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
type Move = Pos;

#[derive(Debug, Clone)]
struct Warehouse {
	moves: Vec<Direction>,
	robot: Pos,
	boxes: HashSet<Pos>,
	walls: HashSet<Pos>,
	size: (usize, usize),
}

impl Warehouse {
	fn new(
		moves: Vec<Direction>,
		robot: Pos,
		boxes: HashSet<Pos>,
		walls: HashSet<Pos>,
		size: (usize, usize),
	) -> Self {
		Self {
			moves,
			robot,
			boxes,
			walls,
			size,
		}
	}
	// could return the resulting grid
	pub fn run(&mut self) {}
	pub fn checksum(&self) -> u64 {
		let mut box_result: u64 = 0;
		dbg!(self.size);
		let (height, _width) = self.size;
		// let (_width, height) = self.size;
		for single_box in &self.boxes {
			dbg!(single_box.0);
			box_result += single_box.0 as u64;
			dbg!(box_result);
			dbg!(height - single_box.1 as usize);
			dbg!(height);
			box_result += ((height - 1) - single_box.1 as usize) as u64 * 100;
			dbg!(box_result);
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
	for (y, row) in grid.iter_rows().enumerate() {
		for (x, &c) in row.enumerate() {
			// if x == 2 && y == 4 {
			// 	dbg!("{:?}", c);
			// }
			if c == item {
				positions.insert(Pos(x as i32, y as i32));
			}
		}
	}
	positions
}

// assert_eq!(grid[(5, 1)], 'O');
// assert_eq!(grid[(4, 4)], '@');
// // <vv>
// dbg!(&movements[0..9]);
// assert_eq!(
// 	&[
// Direction::Left,
// Direction::Down,
// Direction::Down,
// Direction::Right,
// 	],
// 	&movements[0..=3] /* or 0..4 for first four */
// );

const WALL: char = '#';
const BOX: char = 'O';
#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let (grid, movements) = parse(input)?;
	println!("{:?}", grid);
	let robot_start = find_robot(&grid);
	let walls = find_items(&grid, WALL);
	eprintln!("{:?}", walls);
	let boxes = find_items(&grid, BOX);
	assert_eq!(37, walls.len());
	assert_eq!(21, boxes.len());
	assert!(walls.contains(&Pos(2, 4)));
	assert!(boxes.contains(&Pos(1, 3)));
	assert!(boxes.contains(&Pos(1, 4)));
	let mut maze = Warehouse::new(movements, robot_start, boxes, walls, grid.size());
	maze.run(); // we might assign this later
	// 100 * distance from top edge + distance from left (x)
	let box_checksum = maze.checksum();
	Ok(box_checksum.to_string())
}

// fn print_xy_increment(grid: &Grid<char>) {
// 	for
// }
// fn print_yx_increment(grid: &Grid<char>) {
// 	let mut y = 0;
// 	for _ in grid[(y, x)] {
// let mut x = 0;
// for grid[(y, x)] {
// 	eprint!("{}",grid[(y, x)] );
// }
// 	}
// }

#[cfg(test)]
mod tests {
	use std::vec;

	use super::*;

	#[test]
	fn test_stuff() {}
	// 	#[test]
	// 	fn test_g1() {
	// 		let mut grid = grid![[1, 2, 3]
	// [4, 5, 6]
	// [7, 8, 9]];
	// 		grid.flip_cols();
	// 		assert_eq!(grid[(2, 0)], 9);
	// 		assert_eq!(grid.size(), (3, 3));
	// 		eprintln!("default");
	// 		dbg!(&grid);
	// 		assert_eq!(6, grid[(1, 2)]);
	// 		let grid = grid![[1,2,3][4,5,6][7,8,9]];
	// 		dbg!(&grid);
	// 		assert_eq!(grid[(1, 0)], 2);
	// 	}
	// #[test]
	// fn test_g2() {
	// 	// 777 is y_max / lines first
	// 	// 444 is in between them
	// 	// 111 is y_min / lines last
	// 	// we want to access as first the last lines
	// 	let mut grid = grid_cm![];
	// 	grid.insert_col(0, vec![7, 8, 9]);
	// 	grid.insert_col(0, vec![4, 5, 6]);
	// 	grid.insert_col(0, vec![1, 2, 3]);
	// 	// assert_eq!(grid, grid![[7,8,9][4,5,6][1,2,3]]);
	// 	// assert_eq!();
	// 	// y, x
	// 	assert_eq!(grid[(0, 1)], 2);
	// 	dbg!(&grid);
	// 	grid.transpose();
	// 	dbg!(&grid);
	// 	assert_eq!(grid[(1, 0)], 2);
	// 	dbg!(&grid);
	// 	grid.insert_row(3, vec![10]);
	// 	assert_eq!(grid.size(), (3, 3));
	// }
	// #[test]
	// fn test_contains() {
	// 	let mut walls = HashSet::new();
	// 	walls.insert(Pos(2, 4));

	// 	assert!(walls.contains(&Pos(2, 4)));
	// }

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
	fn test_grid() {
		// let (mut grid, _) = parse(input).unwrap();
		let mut grid_rev_stack = grid_cm![
		['.', '.', '.', '.', '.', '.', '#']
		['.', '.', 'O', '.', '.', '.', '#']
		['#', '#', '#', '#', '#', '#', '#']
		];
		// y, x
		assert_eq!(grid_rev_stack[(1, 2)], 'O');
		grid_rev_stack.flip_rows();
		eprintln!("before");
		dbg!(&grid_rev_stack);
		grid_rev_stack.rotate_half();
		dbg!(&grid_rev_stack);
		let mut grid_two = grid![
		[ '#', '#', '#', '#', '#', '#', '#']
		[ '#', '.', '.', '.', 'O', '.', '.']
		[ '#', '.', '.', '.', '.', '.', '.']
		];
		// y, x
		assert_eq!(grid_two[(1, 4)], 'O');
		dbg!(&grid_two);
		let input = "#######
#...O..
#......
";
		let grid = parse_grid(input);
		assert_eq!(grid_two, grid);
		// assert_eq!(expected_grid[(2, 1)], grid[(2, 1)]);
	}
	#[test]
	fn test_checksum_one() {
		let input = "#######
#...O..
#......

<>";
		let (mut grid, movements) = parse(input).unwrap();
		for row in grid.iter_rows() {
			dbg!(row);
		}
		assert_eq!(grid.size(), (grid.rows(), grid.cols()));
		assert_eq!((grid.rows(), grid.cols()), (3, 7));
		let (walls, boxes) = (find_items(&grid, WALL), find_items(&grid, BOX));
		let maze = Warehouse {
			moves: vec![],
			robot: Pos(0, 0),
			boxes,
			walls,
			size: grid.size(),
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
		for row in grid.iter_rows() {
			dbg!(row);
		}
		// assert_eq!(grid.size(), (grid.rows(), grid.cols()));
		// assert_eq!((grid.rows(), grid.cols()), (3, 7));
		let robot_pos = find_robot(&grid);
		// Pos(3, 5)
		assert_eq!(Pos(3, 5), robot_pos);
		let (walls, boxes) = (find_items(&grid, WALL), find_items(&grid, BOX));
		let maze = Warehouse {
			moves: vec![],
			robot: robot_pos,
			boxes,
			walls,
			size: grid.size(),
		};
		let box_check = maze.checksum();
		assert_eq!(10092, box_check);
	}
}
