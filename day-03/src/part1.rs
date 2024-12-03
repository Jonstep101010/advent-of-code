// use nom::*;

fn parse(input: String) -> miette::Result<String> {
	let begin = input.find("mul(").unwrap();
	let outer_end = input.rfind(")").unwrap();
	let mut data = &input[begin..outer_end + 1];
	dbg!(data);
	let mut first = true;
	let mut total: i64 = 0;
	loop {
		if data.contains("mul(") && data.contains(")") && data.contains(",") {
			let start = data.find("mul(").unwrap();
			data = &data[start..];
			dbg!(data);
			let end = data.find(")").unwrap();
			dbg!(&data[0..end + 1]);
			let inside = &data[4..end];
			dbg!(inside);
			if inside.contains("()")
				|| inside.starts_with("mul(")
				|| inside.ends_with(")")
				|| !inside.contains(",")
			{
				data = &data[3..];
			} else {
				let comma_exists = inside.find(",");
				assert!(comma_exists.is_some());
				let comma = comma_exists.unwrap();
				let first_res = inside[..comma].parse::<i64>();
				dbg!(&inside[..comma]);
				let second_res = inside[comma + 1..].parse::<i64>();
				dbg!(&inside[comma + 1..]);
				if first_res.is_ok() && second_res.is_ok() {
					let mul_product = first_res.unwrap() * second_res.unwrap();
					if first {
						total = mul_product;
						first = false;
					} else {
						total += mul_product;
					}
				}
				let data_new = data;
				let new = &data_new[end - inside.len()..].find("mul(");
				if new.is_some() {
					data = &data[new.unwrap()..];
					dbg!(data);
				} else {
					break;
				}
			}
		} else {
			break;
		}
		dbg!(total);
	}
	dbg!(total);
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
	#[test]
	fn test_infinite() -> miette::Result<()> {
		let input = "mul(84,895)who(177,299)";
		assert_eq!("75180", process(input)?);
		Ok(())
	}
}
