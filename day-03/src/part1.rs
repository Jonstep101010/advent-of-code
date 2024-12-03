// use nom::*;

use nom::FindSubstring;

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
			let start = data.find("mul(").unwrap();
			let comma_exists = data.find(",");
			assert!(comma_exists.is_some());
			let comma = comma_exists.unwrap();
			let end = data.find(")").unwrap();
			if start > end || comma + 1 > end || comma < start {
				unreachable!("should not happen: start > end");
			}
			dbg!(&data[start + 4..comma]);
			dbg!(&data[comma + 1..end]);
			if data[start + 4..comma].parse::<i32>().is_ok()
				&& data[comma..end].parse::<i32>().is_ok()
			{
				dbg!(data);
				dbg!(&data[start..]);
				dbg!(&data[start..end]);
				let cur_elem = &data[start + 4..end];
				dbg!(cur_elem);
				// assert!(cur_elem.starts_with("mul("));
				// assert!(cur_elem.ends_with(")"));
				if !cur_elem.contains(",") {
					dbg!(data);
					continue;
				}
				// check that inside is num1,num2
				dbg!(start);
				dbg!(end);
				// dbg!(&cur_elem[start..]);
				let inside = cur_elem;
				dbg!(inside);
				let split_comma: Vec<&str> = inside.split(",").collect();
				dbg!(&split_comma);
				if split_comma.len() == 2 {
					let first_result = split_comma[0].parse::<i32>();
					let second_result = split_comma[1].parse::<i32>();
					if first_result.is_ok() && second_result.is_ok() {
						let to_calc = (
							first_result.clone().unwrap(),
							second_result.clone().unwrap(),
						);
						dbg!(&to_calc.0);
						dbg!(&to_calc.1);
						let mul_product = to_calc.0 * to_calc.1;
						if first {
							total = mul_product;
							first = false;
						} else {
							total += mul_product;
						}
						let cur_slice = &data[end..];
						let bind = cur_slice.find("mul(");
						if bind.is_some() {
							data = &cur_slice[bind.unwrap()..];
						} else {
							break;
						}
						dbg!(data);
					}
				} else {
					let cur_slice = &data[end..];
					let bind = cur_slice.find("mul(");
					if bind.is_some() {
						data = &cur_slice[bind.unwrap()..];
					} else {
						break;
					}
				}
			} else {
				data = &data[start..];
				dbg!(data);
			}
		} else {
			data = &data[start..];
			dbg!(data);
		}
	}
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
