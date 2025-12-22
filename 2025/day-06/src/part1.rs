use itertools::{Itertools, any};
use miette::IntoDiagnostic;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
	let mut nums = vec![];
	for line in _input.lines() {
		let split = line.split_whitespace();
		if any(line.chars(), |c| c == '*' || c == '+') {
			let ops = split.collect_vec();
			let opslen = ops.len();
			return Ok(ops
				.into_iter()
				.enumerate()
				.map(|(i, op)| {
					let cur_range = nums[i..nums.len()].iter().step_by(opslen);
					if op == "*" {
						cur_range.product::<usize>()
					} else {
						cur_range.sum::<usize>()
					}
				})
				.sum::<usize>()
				.to_string());
		}
		for str in split {
			nums.push(str.parse::<usize>().into_diagnostic()?);
		}
	}
	unreachable!()
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
		assert_eq!("4277556", process(input)?);
		Ok(())
	}
}
