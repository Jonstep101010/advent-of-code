use miette::miette;
use tracing::info;

fn parse(input: &str) -> nom::IResult<&str, Vec<(u64, Vec<u64>)>> {
	nom::multi::separated_list1(
		nom::character::complete::line_ending, /* each line has one equation */
		nom::sequence::separated_pair(
			/* "result: n1 n2 n3 n4"  */
			nom::character::complete::u64,
			nom::bytes::complete::tag(": "),
			nom::multi::separated_list1(
				nom::character::complete::space1,
				nom::character::complete::u64,
			),
		),
	)(input)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let (_input, equations) = parse(input).map_err(|err| miette!("failed to parse {}", err))?;
	dbg!(&equations);
	Ok(total_sum.to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
		assert_eq!("3749", process(input)?);
		Ok(())
	}
}
