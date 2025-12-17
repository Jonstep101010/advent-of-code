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
	let mut roll_positions = _input
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
	let mut removed_total = 0;
	loop {
		let remainder: HashSet<IVec2> = roll_positions
			.iter()
			.filter(|&roll| {
				// keep those who can't be taken: inverse part1
				DIRECTIONS
					.iter()
					.filter(|&dir_vec| roll_positions.contains(&(roll + dir_vec)))
					.count() >= 4
			})
			.cloned()
			.collect();
		if remainder.len() == roll_positions.len() {
			break;
		} else {
			removed_total += roll_positions.len() - remainder.len();
			roll_positions = remainder;
		}
	}
	Ok(removed_total.to_string())
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
		assert_eq!("43", process(input)?);
		Ok(())
	}
}
