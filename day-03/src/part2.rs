// use nom::*;

fn parse(input: String) -> miette::Result<String> {
	let mut data = &input[..];
	let mut first = true;
	let mut enabled = true;
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
				if enabled {
					calc_total(inside, &mut first, &mut total);
				}
				let new = &data[end - inside.len()..].find("mul(");
				if new.is_none() {
					break;
				}
				let change_one = &data[end - inside.len()..].find("do()");
				let change_two = &data[end - inside.len()..].find("don't()");
				if change_two.is_some() && new.unwrap() > change_two.unwrap() {
					let new = change_two;
					dbg!(new);
					dbg!(&data[new.unwrap()..]);
					enabled = false;
					dbg!(data);
					eprintln!("switched to disabled!");
				}
				if change_one.is_some() && new.unwrap() > change_one.unwrap() {
					let new = change_one;
					dbg!(new);
					enabled = true;
					dbg!(data);
					eprintln!("enabled!");
				}
				data = &data[new.unwrap()..];
			}
		} else {
			break;
		}
		dbg!(total);
	}
	dbg!(total);
	Ok(total.to_string())
}

fn calc_total(inside: &str, first: &mut bool, total: &mut i64) {
	// assert!(inside.find(",").is_some());
	let comma = inside.find(",").unwrap();
	let first_res = inside[..comma].parse::<i64>();
	dbg!(&inside[..comma]);
	let second_res = inside[comma + 1..].parse::<i64>();
	dbg!(&inside[comma + 1..]);
	if first_res.is_ok() && second_res.is_ok() {
		let mul_product = first_res.unwrap() * second_res.unwrap();
		if *first {
			*total = mul_product;
			*first = false;
		} else {
			*total += mul_product;
		}
	}
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let mut data = String::new();
	for line in input.lines() {
		data.push_str(line);
	}
	parse(data)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
		// 48 = (2*4 + 8*5)
		assert_eq!("48", process(input)?);
		Ok(())
	}
	#[test]
	fn test_infinite() -> miette::Result<()> {
		let input = "mul(84,895)who(177,299)";
		assert_eq!("75180", process(input)?);
		Ok(())
	}
}
