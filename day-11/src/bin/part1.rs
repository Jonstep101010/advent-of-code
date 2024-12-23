use day_11::part1::process;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
	tracing_subscriber::fmt::init();

	let file = include_str!("../../input1.txt");
	let result = process(file, 25 - 1).context("process part 1")?;
	println!("{}", result);
	Ok(())
}
