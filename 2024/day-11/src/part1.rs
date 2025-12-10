use either::Either;
use num_traits::Euclid;
use std::iter::from_fn;

#[tracing::instrument]
pub fn process(input: &str, blinks: usize) -> miette::Result<String> {
	// parse input stones
	let mut stones: Vec<u64> = input
		.split_ascii_whitespace()
		.map(|stone| stone.parse::<u64>().expect("all input shall be valid"))
		.collect();

	// check rules, replace/split/multiply, cache iteration?
	// stones have linear history, never merging
	let mut all_iterations = from_fn(move || {
		let next_stones: Vec<u64> = stones
			.iter()
			.flat_map(|stone| {
				match stone {
					0 => {
						// get the next value: 1
						Either::<[u64; 1], _>::Left([1])
					}
					n if (n.checked_ilog10().unwrap_or(0) + 1) % 2 == 0 => {
						// get the next values by splitting at midpoint
						let split_at = (n.checked_ilog10().unwrap_or(0) + 1) / 2;
						let (left, right) = n.div_rem_euclid(&10u64.pow(split_at));
						// Right denotes we have 2 values (Left has one)
						Either::<[u64; 1], [u64; 2]>::Right([left, right])
					}
					n => Either::<[u64; 1], _>::Left([n * 2024]),
				}
				.into_iter()
			})
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
	#[test]
	fn test_process_example() -> miette::Result<()> {
		let input = "125 17";
		assert_eq!("55312", process(input, 25 - 1)?);
		Ok(())
	}
}
