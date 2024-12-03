#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let mut left = vec![];
	let mut right = vec![];

	for line in input.lines() {
		let mut content = line.split_whitespace();
		left.push(content.next().unwrap().parse::<i32>().unwrap());
		right.push(content.next().unwrap().parse::<i32>().unwrap());
	}
	// val = left[i]
	// multiply val by amount of times it appears in right
	// sum all vals
	let score = left.iter().fold(0, |similarity_acc, &val| {
		similarity_acc + val * right.iter().filter(|&&right_val| right_val == val).count() as i32
	});
	Ok(score.to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_task() -> miette::Result<()> {
		let input = "3 4
4 3
2 5
1 3
3 9
3 3";
		assert_eq!("31", process(input)?);
		Ok(())
	}
}
