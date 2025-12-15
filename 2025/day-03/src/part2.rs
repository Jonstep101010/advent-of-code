use itertools::Itertools;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
	let joltages_total = _input
		.lines()
		.map(|battery_bank| {
			let mut joltages: Vec<char> = vec![];
			let mut offset = 0;
			for i in 0..12 {
				let (idx, first_max) = &battery_bank[offset..(battery_bank.len() - 11 + i)]
					.chars()
					.enumerate()
					.max_set_by_key(|(_, battery)| *battery)
					.first()
					.cloned()
					.unwrap();
				joltages.push(*first_max);
				offset += idx + 1;
			}
			joltages.iter().collect::<String>().parse::<u64>().unwrap()
		})
		.sum::<u64>();
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
