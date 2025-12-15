#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
	let mut joltages_total = 0;
	for battery_bank in _input.lines() {
		let mut joltage_output: [Option<u32>; 12] = [None; 12];
		let mut bank_digits: Vec<u32> = vec![];
		let mut largest_idx = None;
		for battery in battery_bank.chars() {
			let battery = battery.to_digit(10).unwrap();
			if largest_idx.is_none() || battery > bank_digits[largest_idx.unwrap()] {
				largest_idx = Some(bank_digits.len());
			}
			bank_digits.push(battery)
		}
		let mut offset = 0;
		for i in 0..12 {
			let (relative_idx, first_max) = bank_digits[offset..(bank_digits.len() - 11 + i)]
				.iter()
				.enumerate()
				.rev() // last max in reversed
				.max_by_key(|(_, battery)| *battery)
				.unwrap();
			joltage_output[i] = Some(*first_max);
			offset += relative_idx + 1;
		}
		let joltage_total = joltage_output
			.iter()
			.map(|c| c.unwrap())
			.fold(0u64, |acc, num| {
				acc * 10u64.pow(num.to_string().len() as u32) + num as u64
			});
		joltages_total += joltage_total;
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
		assert_eq!("3121910778619", process(input)?);
		Ok(())
	}
}
