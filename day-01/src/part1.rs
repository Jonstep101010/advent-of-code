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
	Ok("".to_string())
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
}
