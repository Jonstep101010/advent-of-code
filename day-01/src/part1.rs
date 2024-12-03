use std::cmp::{max, min};

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
	// get distance between the two values for each index
	eprintln!("{:#?}", &left);
	eprintln!("{:#?}", &right);
	let mut distance = 0;
	for i in 0..left.len() {
		let diff = left[i].abs_diff(right[i]);
		eprintln!("{:#?}", diff);
		distance += diff;
	}
	eprintln!("{:#?}", distance);
	Ok(distance.to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		todo!("haven't built test yet");
		let input = "";
		assert_eq!("", process(input)?);
		Ok(())
	}
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
