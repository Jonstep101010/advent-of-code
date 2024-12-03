// use nom::*;

fn parse(input: String) -> miette::Result<String> {
	let start = input.find("mul(").unwrap();
	let end = input.rfind(")").unwrap();
	let mut data = &input[start..end + 1];
	dbg!(data);
	// find "mul(", from there search for ")"
	// start, end of calc set from ()
	// -> find "," and numbers enclosed
	// mul(number1,number2)
	// 0123       varies  varies
	println!("");
	Ok("".to_string())
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let mut data = String::new();
	for line in input.lines() {
		data.push_str(line);
	}
	let mut total = 0;
	let parsed = parse(data);
	Ok(total.to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
		// 161 = (2*4 + 5*5 + 11*8 + 8*5)
		assert_eq!("161", process(input)?);
		Ok(())
	}
}
