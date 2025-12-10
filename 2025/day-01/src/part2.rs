#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
	todo!("day 01 - part 2");
}

#[cfg(test)]
mod tests {
	use std::{fs::File, io::Read, str};

	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		let input = String::from_utf8(std::fs::read("./input_example.txt").unwrap()).unwrap();
		assert_eq!("6", process(&input)?);
		Ok(())
	}
}
