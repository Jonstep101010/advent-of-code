use miette::Diagnostic;
use std::{error::Error, fmt, num::ParseIntError};

#[derive(Debug, Diagnostic)]
pub enum DialError {
	ParseInt(ParseIntError),
	InvalidInstruction,
}

impl fmt::Display for DialError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			DialError::ParseInt(e) => write!(f, "parse error: {}", e),
			DialError::InvalidInstruction => write!(f, "invalid dial instruction"),
		}
	}
}
impl Error for DialError {}

impl From<ParseIntError> for DialError {
	fn from(e: ParseIntError) -> Self {
		DialError::ParseInt(e)
	}
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, DialError> {
	let mut dial_pos = 50;
	let mut password = 0;
	for line in _input.lines() {
		#[allow(clippy::manual_strip)]
		let action: i32 = if line.starts_with("R") {
			line[1..].parse::<i32>()?
		} else if line.starts_with("L") {
			-line[1..].parse::<i32>()?
		} else {
			return Err(DialError::InvalidInstruction);
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
		assert_eq!("3", process(include_str!("../input_example.txt"))?);
		Ok(())
	}
}
