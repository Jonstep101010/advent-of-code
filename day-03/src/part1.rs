// use nom::*;

fn parse(input: String) -> miette::Result<String> {
	let begin = input.find("mul(").unwrap();
	let outer_end = input.rfind(")").unwrap();
	let mut data = &input[begin..outer_end + 1];
	dbg!(data);
	// find "mul(", from there search for ")"
	// start, end of calc set from ()
	// -> find "," and numbers enclosed
	// mul(number1,number2)
	// 0123       varies  varies
	let mut first = true;
	let mut total = 0;
	loop {
		// if data.starts_with(")") {
		// 	if data.as_bytes().len() == 1 {
		// 		break;
		// 	}
		// 	data = &data[1..]
		// }
		if data.find("mul(").is_some() && data.find(")").is_some() {
			let mut start = data.find("mul(").unwrap();
			data = &data[start..];
			dbg!(data);
			let end = data.find(")").unwrap();
			dbg!(&data[0..end + 1]);
			let inside = &data[4..end];
			dbg!(inside);
			if inside.contains("()") || inside.starts_with("mul(") || inside.ends_with(")") {
				data = &data[5..];
			} else {
				let comma_exists = inside.find(",");
				assert!(comma_exists.is_some());
				let comma = comma_exists.unwrap();
				let first_res = inside[..comma].parse::<i32>();
				dbg!(&inside[..comma]);
				let second_res = inside[comma + 1..].parse::<i32>();
				dbg!(&inside[comma + 1..]);
				if first_res.is_ok() && second_res.is_ok() {
					let mul_product = first_res.unwrap() * second_res.unwrap();
					if first {
						total = mul_product;
						first = false;
					} else {
						total += mul_product;
					}
				} else {
					dbg!(&inside[..comma]);
					dbg!(&inside[comma + 1..]);
				}
				if data[start + 1..].is_empty() {
					assert_eq!(161, total);
					dbg!(total);
					return Ok(total.to_string());
				}
				dbg!(&data[start + 1..]);
				dbg!(&data[start + 1..]);
				let data_new = data;
				let new = &data_new[start + 2..].find("mul(");
				dbg!(&data_new[new.unwrap() + 2..]);
				if new.is_some() {
					start = new.unwrap();
					data = &data[start..];
					dbg!(data);
				} else {
					continue;
				}
			}
		} else {
			break;
		}
		dbg!(total);
	}
	dbg!(total);
	assert_eq!(161, total);
	Ok(total.to_string())
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let mut data = String::new();
	for line in input.lines() {
		data.push_str(line);
	}
	let parsed = parse(data)?;
	Ok(parsed)
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
