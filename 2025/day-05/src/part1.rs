use itertools::Itertools;
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
	let ingredients = lines
		.map(|line| line.parse::<usize>().unwrap())
		.collect_vec();
	// dbg!(&ingredients);
	let total_fresh = ingredients
		.into_iter()
		.filter(|ingredient| {
			itertools::any(&fresh_ranges, |fresh_range| {
				fresh_range.contains(&ingredient)
			})
		})
		.count();
	Ok(total_fresh.to_string())
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
		assert_eq!("3", process(input)?);
		Ok(())
	}
}
