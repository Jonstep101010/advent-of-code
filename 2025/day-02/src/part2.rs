use itertools::Itertools;
use miette::IntoDiagnostic;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
	let mut invalid_ids = 0;
	for range in _input.split(",") {
		let rangesplit: (&str, &str) = range
			.trim()
			.split("-")
			.collect_tuple()
			.expect("two elements");
		let (left, right) = (
			rangesplit.0.parse::<u64>().into_diagnostic()?,
			rangesplit.1.parse::<u64>().into_diagnostic()?,
		);
		for i in left..=right {
			let numstr = i.to_string();
			let mid: usize = numstr.len() / 2;
			if numstr[..mid] == numstr[mid..] {
				invalid_ids += i;
			} else {
				let first = numstr.chars().next().expect("first");
				for (ii, c) in numstr[1..].chars().enumerate() {
					if c == first {
						let maybe_seq = &numstr[..=ii];
						if numstr.len().checked_rem(maybe_seq.len()) == Some(0)
							&& maybe_seq.repeat(numstr.len() / maybe_seq.len()) == numstr
						{
							// println!("found {maybe_seq} in {numstr}");
							invalid_ids += i;
						}
						break;
					}
				}
			}
		}
	}
	Ok(invalid_ids.to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
		assert_eq!("4174379265", process(input)?);
		Ok(())
	}
}
