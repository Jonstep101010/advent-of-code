#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	dbg!(input);
	let result = 0;
	Ok(result.to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		let input = "2333133121414131402";
		assert_eq!("1928", process(input)?);
		Ok(())
	}
}
