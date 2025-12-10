#![warn(clippy::pedantic)]

use num_traits::Euclid;
use std::collections::HashMap;

trait Cache {
	fn entry_increment_count(&mut self, entry: u64, stone_count: u64);
}

impl Cache for HashMap<u64, u64> {
	fn entry_increment_count(&mut self, entry: u64, stone_count: u64) {
		self.entry(entry)
			.and_modify(|v| {
				// increment by 1
				*v += stone_count;
			})
			// if nonexistent, insert 1
			.or_insert(stone_count);
	}
}

#[tracing::instrument]
pub fn process(input: &str, blinks: usize) -> miette::Result<String> {
	// parse input stones
	let stones: Vec<u64> = input
		.split_ascii_whitespace()
		.map(|stone| stone.parse::<u64>().expect("all input shall be valid"))
		.collect();

	// store stones, associated count
	let mut cache: HashMap<u64, u64> = HashMap::default();
	for &stone in &stones {
		cache.entry_increment_count(stone, 1);
	}

	for _ in 0..blinks {
		let mut new_cache: HashMap<u64, u64> = HashMap::new();

		for (stone, &count) in &cache {
			match stone {
				0 => {
					// get the next value: 1
					new_cache.entry_increment_count(1, count);
				}
				n if (n.checked_ilog10().unwrap_or(0) + 1) % 2 == 0 => {
					// get the next values by splitting at midpoint
					let split_at = (n.checked_ilog10().unwrap_or(0) + 1) / 2;
					let (left, right) = n.div_rem_euclid(&10u64.pow(split_at));
					new_cache.entry_increment_count(left, count);
					new_cache.entry_increment_count(right, count);
				}
				n => new_cache.entry_increment_count(n * 2024, count),
			}
		}
		cache = new_cache;
	}
	Ok(cache.values().sum::<u64>().to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process_example() -> miette::Result<()> {
		let input = "125 17";
		assert_eq!("55312", process(input, 25)?);
		Ok(())
	}
}
