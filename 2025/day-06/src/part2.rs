use itertools::{Itertools, max};
use pad::PadStr;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
	let mut lines = _input
		.lines()
		.map(|line| line.trim_matches('\n'))
		.collect_vec();
	let ops = lines
		.pop()
		.expect("operators trailing")
		.chars()
		.collect_vec();
	let max_len = max(lines.iter().map(|line| line.len())).expect("maximum length");
	let rows = lines
		.iter_mut()
		.map(|line| line.pad_to_width_with_alignment(max_len, pad::Alignment::Left))
		.collect_vec();
	let mut cur_digits: Vec<usize> = Vec::with_capacity(lines.len());
	let total: usize = (0..ops.len())
		.rev()
		.filter_map(|col| {
			let digits: String = rows
				.iter()
				.filter_map(|row| row.chars().nth(col).filter(|&digit| digit != ' '))
				.collect();
			if digits.is_empty() {
				cur_digits.clear();
			} else {
				cur_digits.push(digits.parse::<usize>().unwrap());
			}
			match ops[col] {
				'+' => Some(cur_digits.iter().sum::<usize>()),
				'*' => Some(cur_digits.iter().product::<usize>()),
				_ => None,
			}
		})
		.sum();
	Ok(total.to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";
		assert_eq!("3263827", process(input)?);
		Ok(())
	}
}
