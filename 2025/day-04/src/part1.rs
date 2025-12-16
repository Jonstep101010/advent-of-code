use std::collections::HashSet;

use glam::IVec2;
const DIRECTIONS: [IVec2; 8] = [
	IVec2::NEG_X,
	IVec2::NEG_Y,
	IVec2::X,
	IVec2::Y,
	IVec2::ONE,
	IVec2::NEG_ONE,
	IVec2::new(1, -1),
	IVec2::new(-1, 1),
];

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
	let roll_positions = _input
		.lines()
		.enumerate()
		.flat_map(|(y, line)| {
			line.chars().enumerate().filter_map(move |(x, c)| {
				if c == '@' {
					// dbg!(&x, &y);
					Some(IVec2::new(x as i32, y as i32))
				} else {
					None
				}
			})
		})
		.collect::<HashSet<IVec2>>();

	let paper_rolls_inaccessible = roll_positions
		.iter()
		.filter(|&roll| {
			// find nearest neighbors at DIRECTIONS
			DIRECTIONS
				.iter()
				.filter(|&dir_vec| roll_positions.contains(&(roll + dir_vec)))
				.count() >= 4
		})
		.count();
	Ok((roll_positions.len() - paper_rolls_inaccessible).to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
		assert_eq!("13", process(input)?);
		Ok(())
	}
}
