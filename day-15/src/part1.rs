use grid::{Grid, GridRowIter, grid};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
	Up,
	Right,
	Down,
	Left,
}

fn parse(input: &str) -> miette::Result<(Grid<char>, Vec<Direction>)> {
	let (g, ins) = input.split_once("\n\n").unwrap();
	// an x, y grid (row-major)
	let mut grid = grid![];
	for row in g.lines() {
		let items = row.chars().collect_vec();
		dbg!(&items);
		grid.push_row(items);
	}
	dbg!(&grid);
	let instructions = ins
		.replace('\n', "")
		.chars()
		.map(|c| match c {
			'^' => Direction::Up,
			'>' => Direction::Right,
			'v' => Direction::Down,
			'<' => Direction::Left,
			_ => panic!("Invalid direction character"),
		})
		.collect_vec();
	Ok((grid, instructions))
}

// assert_eq!(grid[(5, 1)], 'O');
// assert_eq!(grid[(4, 4)], '@');
// // <vv>
// dbg!(&movements[0..9]);
// assert_eq!(
// 	&[
// 		Direction::Left,
// 		Direction::Down,
// 		Direction::Down,
// 		Direction::Right,
// 	],
// 	&movements[0..=3] /* or 0..4 for first four */
// );
#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let (grid, movements) = parse(input)?;
	todo!();
	// Ok(box_checksum.to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

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
}
