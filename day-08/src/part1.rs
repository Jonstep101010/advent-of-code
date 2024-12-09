use glam::IVec2;
use itertools::Itertools;
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
	let height = input.lines().count();
	let width = input.lines().next().unwrap().len();
	let (bound_horizontal, bound_vertical) = (0..width as i32, 0..height as i32);

	let (_input, mut parsing_result) =
		parse(Span::new(input)).map_err(|err| miette!("failed to parse: {}", err))?;
	parsing_result.sort_by(|a, b| a.1.cmp(&b.1));
	// we want to get in a row the same frequencies,
	// check for each of them their diff and possible resulting antinodes
	let antinode_count = parsing_result
		.chunk_by(|a, b| a.1 == b.1)
		.flat_map(|chunk| {
			itertools::Itertools::combinations(chunk.iter(), 2)
				.flat_map(|antennas| {
					// antennas: combination of 2 points of same type (same char & case/num)
					let diff = antennas[0].0 - antennas[1].0;
					[antennas[0].0 + diff, antennas[1].0 - diff]
				})
				.filter(|position| {
					bound_horizontal.contains(&position.x) && bound_vertical.contains(&position.y)
				}) //.inspect(|v| {dbg!(v);})
		})
		.unique()
		.count();
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
