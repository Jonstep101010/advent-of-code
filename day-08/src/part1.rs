use glam::IVec2;
use miette::miette;
use nom::{
	AsChar, IResult, bytes::complete::take_till, character::complete::satisfy, multi::many1,
	sequence::preceded,
};
use nom_locate::{LocatedSpan, position};

pub type Span<'a> = LocatedSpan<&'a str>;

fn alphanum_position(input: Span) -> IResult<Span, (IVec2, char)> {
	let (input, position) = position(input)?;
	let (x, y) = (
		position.get_column() as i32 - 1,
		position.location_line() as i32 - 1,
	);
	let (input, c) = satisfy(|c| c.is_alphanum())(input)?;
	Ok((input, (IVec2::new(x, y), c)))
}

fn parse(input: Span) -> IResult<Span, Vec<(IVec2, char)>> {
	let prec = preceded(
		take_till(|c: char| nom::AsChar::is_alphanum(c)), // get rid of all these
		alphanum_position,                                // keep all of this
	);
	many1(prec)(input)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let (_input, parsing_result) =
		parse(Span::new(input)).map_err(|err| miette!("failed to parse: {}", err))?;
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
