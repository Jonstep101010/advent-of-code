use itertools::Itertools;
use miette::IntoDiagnostic;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
	let mut nums = vec![];
	let mut ops: Option<Vec<&str>> = None;
	for line in _input.lines() {
		let split = line.split_whitespace();
		if itertools::any(line.chars(), |c| c == '*' || c == '+') {
			ops = Some(split.collect_vec());
			break;
		} else {
			split.for_each(|str| {
				let n = str.parse::<usize>().into_diagnostic().expect("valid n");
				nums.push(n);
			});
		}
	}
	let mut total: usize = 0;
	let opslen = ops.clone().expect("operator trailing").iter().len();
	for (i, op) in ops.expect("trailing ops").into_iter().enumerate() {
		let cur = (i..nums.len())
			.step_by(opslen)
			.map(|ii| nums[ii])
			.collect_vec();
		total += if op == "*" {
			cur.iter().product::<usize>()
		} else {
			cur.iter().sum::<usize>()
		};
	}
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
		assert_eq!("4277556", process(input)?);
		Ok(())
	}
}
