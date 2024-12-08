use miette::miette;
use nom::{
	IResult, bytes::complete::take_till, character::complete::satisfy, multi::many1,
	sequence::preceded,
};

fn parse(input: &str) -> IResult<&str, Vec<char>> {
	let prec = preceded(
		take_till(|c: char| nom::AsChar::is_alphanum(c)), // get rid of all these
		satisfy(|c| nom::AsChar::is_alphanum(c)),         // keep all of this
	);
	many1(prec)(input)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let (_input, parsing_result) =
		parse(input).map_err(|err| miette!("failed to parse: {}", err))?;
	dbg!(parsing_result);
	let antinode_count = 0;
	Ok(antinode_count.to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
		assert_eq!("14", process(input)?);
		Ok(())
	}
}
