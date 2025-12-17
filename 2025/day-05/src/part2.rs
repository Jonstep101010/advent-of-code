use itertools::Itertools;
use miette::IntoDiagnostic;
use rangemap::range_inclusive_set;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
	let mut lines = _input.lines();
	let mut min_max_set = range_inclusive_set![];
	while let Some(line) = lines.next()
		&& !line.is_empty()
	{
		let (start, end) = line
			.split("-")
			.map(|elem| elem.parse::<usize>().into_diagnostic())
			.collect_tuple()
			.unwrap();
		min_max_set.insert(start?..=end?);
	}

	Ok(min_max_set
		.iter()
		.map(|contained| contained.end() + 1 - contained.start())
		.sum::<usize>()
		.to_string())
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
