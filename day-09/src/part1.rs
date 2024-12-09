fn get_checksum(compressed: &str) -> usize {
	let mut checksum = 0;
	for (i, val) in compressed.chars().enumerate() {
		if val.is_ascii_digit() {
			checksum += i * (val.to_digit(10).unwrap() as usize);
		}
	}
	checksum
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	dbg!(input);
	let compressed = todo!();
	let checksum = get_checksum(compressed);
	Ok(checksum.to_string())
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
	#[test]
	fn test_checksum() -> miette::Result<()> {
		let input = "0099811188827773336446555566..............";
		assert_eq!("1928", get_checksum(input).to_string());
		Ok(())
	}
}
