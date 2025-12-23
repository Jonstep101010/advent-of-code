use std::collections::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let mut lines_it = input.lines().enumerate();
	let manifold_pos = lines_it
		.find(|(_y, line)| line.contains('S'))
		.and_then(|(y, line)| line.find('S').map(|x| (x, y)))
		.unwrap();

	Ok(lines_it
		.fold(
			HashMap::from([(manifold_pos.0, 1); 1]),
			|hm_acc_pos, (_y, line)| {
				let mut new_pos: HashMap<usize, usize> = HashMap::default();
				hm_acc_pos.iter().for_each(|(&x, &count)| {
					if line.chars().nth(x) == Some('^') {
						*new_pos.entry(x - 1).or_insert(0) += count;
						*new_pos.entry(x + 1).or_insert(0) += count;
					} else {
						*new_pos.entry(x).or_insert(0) += count;
					}
				});
				new_pos
			},
		)
		.values()
		.sum::<usize>()
		.to_string())
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
		assert_eq!("40", process(input)?);
		Ok(())
	}
}
