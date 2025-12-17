#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
	todo!("day 01 - part 1");
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
