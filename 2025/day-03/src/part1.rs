use std::num::ParseIntError;

use miette::IntoDiagnostic;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
	let mut joltages_total = 0;
	for battery_bank in _input.lines() {
		let mut largest_overall = None;
		let mut second_largest = battery_bank.chars().nth(1).unwrap().to_digit(10);
		let mut bank_digits: Vec<u32> = vec![];
		let mut largest_idx = 0;
		for battery in battery_bank.chars() {
			let battery = battery.to_digit(10).unwrap();
			if largest_overall.is_none() || battery > largest_overall.unwrap() {
				if largest_overall.is_some() {
					second_largest = largest_overall;
				}
				largest_idx = bank_digits.len();
				largest_overall = Some(battery);
			}
			bank_digits.push(battery)
		}
		fn create_digit(first: Option<u32>, second: Option<u32>) -> Result<u64, ParseIntError> {
			format!("{}{}", first.unwrap(), second.unwrap()).parse()
		}
		let mut largest_joined = if largest_overall == bank_digits.first().copied() {
			create_digit(largest_overall, second_largest)
		} else {
			create_digit(second_largest, largest_overall)
		}
		.into_diagnostic()?;
		for second_digit in &bank_digits[largest_idx + 1..] {
			largest_joined = match create_digit(largest_overall, Some(*second_digit)) {
				Ok(maybe_larger) if maybe_larger > largest_joined => maybe_larger,
				Ok(_) => largest_joined,
				Err(e) => Err(e).into_diagnostic()?,
			}
		}
		joltages_total += largest_joined
	}
	Ok(joltages_total.to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		let input = "987654321111111
811111111111119
234234234234278
818181911112111";
		assert_eq!("357", process(input)?);
		Ok(())
	}
}
