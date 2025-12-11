use miette::IntoDiagnostic;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
	let mut dial_pos = 50;
	let mut password = 0;
	for line in _input.lines() {
		let action_abs = line[1..].parse::<i32>().into_diagnostic()?;
		let (full, partial) = (action_abs.div_euclid(100), action_abs.rem_euclid(100));
		dial_pos = {
			let new = if line.starts_with("R") {
				dial_pos + partial
			} else {
				dial_pos - partial
			};
			password += full
				+ if !(dial_pos == 0 || (0 < new && new < 100)) {
					1
				} else {
					0
				};
			new.rem_euclid(100)
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
		assert_eq!("6", process(&input)?);
		Ok(())
	}
}
