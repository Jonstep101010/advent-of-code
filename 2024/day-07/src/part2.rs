use itertools::Itertools;
use miette::miette;

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

const OPERATORS: [char; 3] = ['*', '+', '|'];
use rayon::prelude::*;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let (_input, equations) = parse(input).map_err(|err| miette!("failed to parse {}", err))?;

	// for explanations see part1
	let total_sum: u64 = equations
		.par_iter()
		.filter_map(|(possible_result, numbers)| {
			(0..numbers.len() - 1) // for (operator count)
				.map(|_| OPERATORS)
				.multi_cartesian_product()
				.any(|sequence_output| {
					let mut s = sequence_output.iter();
					let current_result = numbers
						.iter()
						.copied()
						.reduce(|current, next| match s.next().unwrap() {
							'*' => current * next,
							'+' => current + next,
							'|' => format!("{current}{next}").parse::<u64>().unwrap(),
							_ => panic!("invalid operation"),
						})
						.unwrap();
					*possible_result == current_result
				})
				.then_some(possible_result)
		})
		.sum();
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
		assert_eq!("11387", process(input)?);
		Ok(())
	}
}
