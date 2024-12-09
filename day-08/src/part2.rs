use glam::IVec2;
use itertools::Itertools;
use miette::miette;
use nom::{
	IResult, bytes::complete::take_till, character::complete::satisfy, multi::many1,
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
	let (input, c) = satisfy(
		nom::AsChar::is_alphanum, /* same as |c| c.is_alphanum() */
	)(input)?;
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
	let (range_x, range_y) = (
		0..input.lines().next().unwrap().len() as i32, //width
		0..input.lines().count() as i32,               //height
	);

	let all_antennas_by_freq = parse(Span::new(input))
		// order by frequency with mut parsing_result: freq_0, freq_0, freq_1, freq_1
		.map(|(_, mut parsing_result)| {
			parsing_result.sort_by(|(_, freq_a), (_, freq_b)| freq_a.cmp(freq_b));
			parsing_result
		})
		.map_err(|err| miette!("failed to parse: {}", err))?;

	// check for each of them their diff and possible resulting antinodes inside the grid
	let antinode_count = all_antennas_by_freq
		.chunk_by(|(_, freq_a), (_, freq_b)| freq_a == freq_b)
		.flat_map(|frequency| {
			// for antennas of same frequency
			frequency
				.iter()
				.tuple_combinations() // get combinations
				.flat_map(|(&(pa, _), &(pb, _))| {
					let diff = pa - pb;
					// this combinations' antinodes and successors
					[std::iter::successors(Some(pa), |position: &IVec2| {
						let new_position = position + diff;
						(range_x.contains(&position.x) && range_y.contains(&position.y))
							.then_some(new_position)
					})
					.chain(std::iter::successors(Some(pb), |position: &IVec2| {
						let new_position = position - diff;
						(range_x.contains(&position.x) && range_y.contains(&position.y))
							.then_some(new_position)
					}))
					.collect::<Vec<_>>()]
				})
				.flatten()
				.filter(|position| range_x.contains(&position.x) && range_y.contains(&position.y)) // keep only antinodes within bounds
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
		assert_eq!("34", process(input)?);
		Ok(())
	}
}
