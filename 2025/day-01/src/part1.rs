use miette::Error;
#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
	let mut dial_pos = 50;
	let mut password = 0;
	for line in _input.lines() {
		#[allow(clippy::manual_strip)]
		let action: i32 = if line.starts_with("R") {
			line[1..]
				.parse::<i32>()
				.map_err(|e| Error::msg(e.to_string()))?
		} else if line.starts_with("L") {
			-line[1..]
				.parse::<i32>()
				.map_err(|e| Error::msg(e.to_string()))?
		} else {
			return Err(Error::msg("invalid dial instruction"));
		};
		dial_pos = (dial_pos + action) % 100;
		if dial_pos == 0 {
			password += 1;
		}
	}
	Ok(password.to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		let input = String::from_utf8(std::fs::read("./input_example.txt").unwrap()).unwrap();
		assert_eq!("3", process(&input)?);
		Ok(())
	}
}
