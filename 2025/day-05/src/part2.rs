use std::ops::RangeBounds;

use itertools::{Itertools, any};
use miette::IntoDiagnostic;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
	let mut fresh_ranges = vec![];
	let mut lines = _input.lines();
	while let Some(line) = lines.next()
		&& !line.is_empty()
	{
		let split = line.split("-").collect_vec();
		let range = std::ops::RangeInclusive::new(
			split[0].parse::<usize>().into_diagnostic()?,
			split[1].parse::<usize>().into_diagnostic()?,
		);
		fresh_ranges.push(range);
	}
	// dbg!(&fresh_ranges);
	let min_start = fresh_ranges
		.iter()
		.map(|range_lol| {
			// something
			*range_lol.start()
		})
		.min()
		.unwrap();
	let max_end = fresh_ranges
		.iter()
		.map(|range_lol| {
			// something
			*range_lol.end()
		})
		.max()
		.unwrap();
	let mut contained = max_end + 1 - min_start;
	for i in min_start..=max_end {
		if !any(&fresh_ranges, |range_fresh| range_fresh.contains(&i)) {
			contained -= 1;
		}
	}

	// dbg!(&ingredients);
	// let total_fresh = ingredients
	// 	.into_iter()
	// 	.filter(|ingredient| {
	// 		itertools::any(&fresh_ranges, |fresh_range| {
	// 			fresh_range.contains(&ingredient)
	// 		})
	// 	})
	// 	.count();
	Ok(contained.to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
		assert_eq!("14", process(input)?);
		Ok(())
	}
}
