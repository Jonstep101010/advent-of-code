#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
	let total = 0;
	Ok(total.to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
		assert_eq!("36", process(input)?);
		Ok(())
	}
}
