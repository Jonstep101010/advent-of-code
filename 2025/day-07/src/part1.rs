use std::collections::HashSet;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let manifold_pos = input
		.lines()
		.enumerate()
		.find(|(_y, line)| line.contains('S'))
		.and_then(|(y, line)| line.find('S').map(|x| (x, y)));
	let mut beam_positions: HashSet<(usize, usize)> = HashSet::from([manifold_pos.unwrap()]);
	let mut splits = 0;
	for (y, line) in input.lines().enumerate() {
		for (x, c) in line.chars().enumerate() {
			match (c, beam_positions.contains(&(x, y.wrapping_sub(1)))) {
				('^', true) => {
					splits += (beam_positions.insert((x - 1, y))
						| beam_positions.insert((x + 1, y))) as usize;
				}
				(_, true) => {
					beam_positions.insert((x, y));
				}
				(_, _) => {}
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
