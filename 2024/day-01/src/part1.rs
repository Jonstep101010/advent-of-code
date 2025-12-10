#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let mut left = vec![];
	let mut right = vec![];

	for line in input.lines() {
		let mut content = line.split_whitespace();
		left.push(content.next().unwrap().parse::<i32>().unwrap());
		right.push(content.next().unwrap().parse::<i32>().unwrap());
		eprintln!("{:#?}", &left);
		eprintln!("{:#?}", &right);
	}
	left.sort();
	right.sort();
	// get distance between the two values for each index, sum them up
	eprintln!("{:#?}", &left);
	eprintln!("{:#?}", &right);
	let distance: u32 = left
		.iter()
		.zip(right.iter())
		.map(|(l, r)| l.abs_diff(*r))
		.sum();
	eprintln!("{:#?}", distance);
	Ok(distance.to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_basic() {
		let input = "46669   36559
		54117   62675
25659   15179";
		process(input).unwrap();
	}
	#[test]
	fn test_task() -> miette::Result<()> {
		let input = "3 4
4 3
2 5
1 3
3 9
3 3";
		assert_eq!("11", process(input)?);
		Ok(())
	}
}
