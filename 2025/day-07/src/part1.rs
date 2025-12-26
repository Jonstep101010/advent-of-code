use std::collections::HashSet;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let mut lines_it = input.lines().enumerate();
	let manifold_pos = lines_it
		.next()
		.and_then(|(y, line)| line.find('S').map(|x| (x, y)))
		.unwrap();
	let beam_positions: HashSet<(usize, usize)> = HashSet::from([manifold_pos]);
	let (splits, _) = lines_it.fold(
		(0, beam_positions),
		|(splits, mut beam_positions), (y, line)| {
			(
				splits
					+ line
						.chars()
						.enumerate()
						.filter_map(|(x, c)| {
							match beam_positions.contains(&(x, y.wrapping_sub(1))) {
								true if c == '^' => Some(usize::from(
									beam_positions.insert((x - 1, y))
										| beam_positions.insert((x + 1, y)),
								)),
								true => {
									beam_positions.insert((x, y));
									None
								}
								_ => None,
							}
						})
						.sum::<usize>(),
				beam_positions,
			)
		},
	);
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
