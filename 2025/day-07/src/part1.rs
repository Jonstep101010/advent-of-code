use std::collections::HashSet;

use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let manifold_pos = input
		.lines()
		.enumerate()
		.find(|(_y, line)| line.contains('S'))
		.map(|(y, line)| line.find('S').and_then(|x| Some((x, y))))
		.flatten();
	let mut beam_positions: HashSet<(usize, usize)> = HashSet::new();
	debug_assert!(beam_positions.insert(manifold_pos.unwrap()));
	let mut splits = 0;
	for (y, line) in input.lines().enumerate() {
		for (x, c) in line.chars().enumerate() {
			if c == '^' && beam_positions.contains(&(x, y - 1)) {
				beam_positions.insert((x - 1, y));
				beam_positions.insert((x + 1, y));
				splits += 1;
			} else if beam_positions.contains(&(x, y.wrapping_sub(1))) {
				beam_positions.insert((x, y));
			}
		}
	}
	Ok(splits.to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
		assert_eq!("21", process(input)?);
		Ok(())
	}
}
