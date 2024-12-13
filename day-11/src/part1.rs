// fn change_stones(stones: &[u64]) -> Vec<u64> {
// 	stones.iter().enumerate().map(|(idx, stone)| {
// 		if stone
// 	})
// }
use either::Either;
use std::iter::from_fn;

#[tracing::instrument]
pub fn process(input: &str, blinks: usize) -> miette::Result<String> {
	// parse input stones
	let mut stones: Vec<u64> = input
		.split_ascii_whitespace()
		.map(|stone| stone.parse::<u64>().expect("all input shall be valid"))
		.collect();
	// assert_eq!(vec![0, 1, 10, 99, 999], stones);

	// check rules, replace/split/multiply, cache iteration?
	// stones have linear history, never merging
	// use some sort of recursion iteration
	let mut all_iterations = from_fn(move || {
		let next_stones: Vec<u64> = stones
			.iter()
			.map(|stone| {
				match stone {
					0 => {
						// get the next value: 1
						Either::<[u64; 1], _>::Left([1])
					}
					// get the next values by splitting
					n if (n.checked_ilog10().unwrap_or(0) + 1) % 2 == 0 => {
						// split at midpoint
						let split_at: usize = (n.checked_ilog10().unwrap_or(0) + 1 / 2) as usize;
						// use a String to be able to work on the value
						let split = n.to_string();
						let split = split.split_at(split_at);
						dbg!(split);
						let (left, right) = (
							split.0.parse::<u64>().expect("parsing u64 shall not fail"),
							split.0.parse::<u64>().expect("parsing u64 shall not fail"),
						);
						// Right denotes we have 2 values (Left has one)
						Either::<[u64; 1], [u64; 2]>::Right([left, right])
					}
					n => Either::<[u64; 1], _>::Left([n * 2024]),
				}
				.into_iter()
			})
			.flatten()
			.collect();
		stones = next_stones.clone();
		// Some(stones)
		Some(next_stones)
	});
	// store iterations for each stone/run all and get iteration
	let num_stones = all_iterations.nth(blinks).unwrap().len();
	Ok(num_stones.to_string())
	// Ok("0".to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		let input = "0 1 10 99 999";
		// assert_eq!(at_iteration, "1 2024 1 0 9 9 2021976")
		assert_eq!("7", process(input, 0)?);
		Ok(())
	}
	fn test_process_example() -> miette::Result<()> {
		let input = "125 17";
		assert_eq!("55312", process(input, 25 - 1)?);
		Ok(())
	}
}
