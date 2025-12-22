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
	let mut calcs: Vec<Vec<usize>> = vec![];
	let mut total: usize = 0;
	let opslen = ops.clone().expect("operator trailing").iter().len();
	for (i, op) in ops.expect("trailing ops").into_iter().enumerate() {
		dbg!(&calcs);
		calcs.push(vec![nums[i]]);
		dbg!(&calcs);
		(opslen + i..nums.len())
			.step_by(opslen)
			.for_each(|ii| calcs[i].push(nums[ii]));
		total += if op == "*" {
			calcs[i].iter().product::<usize>()
		} else {
			calcs[i].iter().sum::<usize>()
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
