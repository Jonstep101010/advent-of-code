pub type TokenCost = u64;

fn claw_prize(/* location information */) -> Option<TokenCost> {
	let mut presses_a = 0;
	let mut presses_b = 0;

	if presses_a <= 100 && presses_b <= 100 {
		// if possible in a, b <= 100
		Some(presses_a + presses_b)
	} else {
		// no possibility of reaching prize
		None
	}
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	// parse input into usable data
	// x and y for each button, prize location
	// IVec2?

	// use pathfinding to map with button combinations
	// (max 100 presses per button)
	// check which are possible: Some(token_cost), None (not possible)
	// sum up all possible prizes' token cost

	// minimum tokens to get all possible prizes
	let required_tokens: u64 = 0;
	Ok(required_tokens.to_string())
}

#[cfg(test)]
mod tests {
	use miette::Error;

	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
		assert_eq!("480", process(input)?);
		Ok(())
	}
	#[test]
	fn test_first_example() -> miette::Result<()> {
		let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400";
		assert_eq!("280", process(input)?);
		Ok(())
	}
	#[test]
	fn test_second_impossible() -> miette::Result<()> {
		// there are no possible combinations for reaching the prize
		let input = "Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176";
		assert_eq!(mietteError, claw_prize(input)?);
		Ok(())
	}
}
