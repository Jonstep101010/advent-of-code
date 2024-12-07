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

const OPERATORS: [char; 2] = ['*', '+'];

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let (_input, equations) = parse(input).map_err(|err| miette!("failed to parse {}", err))?;
	// dbg!(&equations);

	println!(" - - - ");

	let total_sum: u64 = equations
		.into_iter()
		.filter_map(|(possible_result, numbers)| {
			// all stuff here
			// get number of operators(numbers), operators(constants)
			let num_operators = numbers.len() - 1;
			(0..num_operators)
				.map(|_ /*ignore - we always need operators*/| OPERATORS)
				/* https://docs.rs/itertools/latest/itertools/trait.Itertools.html#method.multi_cartesian_product */
				.multi_cartesian_product() /* get any/all items, as we need to check if they can be valid */
				.any(|sequence_output| {/* todo: refactor (take closure for now) can_produce_result()*/
					let mut s = sequence_output.iter();
					/* iterate over, reduce by applying op: https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.reduce */
					possible_result == numbers.iter().reduce(|lhs_acc/* first, second */, rhs_elem/* second*/| {
						match s.next().unwrap() {
							'*' => lhs_acc * rhs_elem,
							'+' => lhs_acc + rhs_elem,
							_ => panic!("invalid operation")
						}
					}).unwrap()
				})
				.then_some(possible_result)
		})
		.sum(); // we want to get all numbers
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
