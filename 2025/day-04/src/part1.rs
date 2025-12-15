#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
	let lines = _input.lines();
	let linecount = lines.count();
	dbg!(&linecount);
	// parse into array with locations of IVec2
	let paper_rolls_accessible = 0;
	Ok(paper_rolls_accessible.to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
		assert_eq!("13", process(input)?);
		Ok(())
	}
}
