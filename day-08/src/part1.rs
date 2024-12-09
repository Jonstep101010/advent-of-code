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
	let (_input, mut parsing_result) =
		parse(Span::new(input)).map_err(|err| miette!("failed to parse: {}", err))?;
	// dbg!(&parsing_result);
	parsing_result.sort_by(|a, b| a.1.cmp(&b.1));
	// we want to get in a row the same frequencies
	let antinode_positions = parsing_result
		.chunk_by(|a, b| a.1 == b.1)
		.into_iter()
		.map(|chunk| {
			dbg!(chunk[0]);
			itertools::Itertools::combinations(chunk.iter(), 2)
				.map(|antennas| {
					// antennas: combination of 2 points of same type (same char & case/num)
					// dbg!(antennas);
					antennas[0].0 - antennas[1].0
				})
				.count()
		})
		.collect::<Vec<usize>>();
	// dbg!(&parsing_result);
	Ok(antinode_positions.len().to_string())
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
