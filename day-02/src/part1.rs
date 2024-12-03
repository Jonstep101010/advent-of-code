use std::{cmp::Ordering, collections::HashSet, fmt::Result};

use miette::Error;
///
/// example
/// ```
/// use day_02::part1::has_duplicates;
/// assert_eq!(true, has_duplicates(&vec![8, 6, 4, 4, 1]));
/// assert_eq!(true, has_duplicates(&vec![8, 6, 4, 1, 4]));
/// assert_eq!(false, has_duplicates(&vec![8, 6, 4, 1, 3]));
/// ```
pub fn has_duplicates(vec: &Vec<i32>) -> bool {
	let mut seen = HashSet::new();
	for &value in vec {
		if !seen.insert(value) {
			return true;
		}
	}
	false
}

// check single report
pub fn check_safe(report: &Vec<i32>) -> miette::Result<String> {
	// check if duplicate levels in report
	if has_duplicates(report) {
		// dbg!(&report);
		return Err(Error::msg("duplicates"));
	}
	let mut sorted_report = report.clone();
	sorted_report.sort();
	let mut unsorted_report = sorted_report.clone();
	unsorted_report.reverse();
	// dbg!(&report);
	// dbg!(sorted_report.clone());
	// dbg!(unsorted_report.clone());
	let order = match &report {
		sorted_report => Ordering::Greater,
		unsorted_report => Ordering::Less,
		_ => {
			// unimplemented!("there is no order!")
			return Err(Error::msg("duplicates"));
		}
	};
	if *report != sorted_report && *report != unsorted_report || report.len() != sorted_report.len()
	{
		return Err(Error::msg("fatal unsorted"));
	}
	let clone = report.clone();
	let mut levels = clone.chunks(2);
	#[allow(clippy::iter_next_loop)]
	for pair in levels.next() {
		if order
			!= if pair.windows(2).all(|w| w[0] < w[1]) {
				Ordering::Less // Ascending order
			} else if pair.windows(2).all(|w| w[0] > w[1]) {
				Ordering::Greater // Descending order
			} else {
				// Err(Error::msg("there is no order!"))
				unimplemented!("error fatal");
				// Ordering::Equal // Not sorted
			} {}
		if pair.windows(2).all(|w| w[0].abs_diff(w[1]) < 4) {
			return Err(Error::msg("diff > 3"));
		}
		eprintln!("pair: {:?}", pair);
	}
	// check distance between the two numbers: <= 3
	// check ordering alignment (does not change)

	Ok("".to_string())
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let mut reports = vec![];
	for line in input.lines() {
		let report_vec: Vec<_> = line.split_whitespace().collect();
		let levels = report_vec
			.iter()
			.map(|x| x.parse::<i32>().unwrap())
			.collect::<Vec<i32>>();
		reports.push(levels);
	}
	let mut safe_reports = 0;
	for report in &mut reports {
		match check_safe(&report) {
			Ok(_) => {
				safe_reports += 1;
			}
			Err(e) => {
				println!("Error: {}", e);
			}
		}
	}
	Ok(safe_reports.to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
		assert_eq!("2", process(input)?);
		Ok(())
	}
	#[test]
	fn test_basic() -> miette::Result<()> {
		let input = "44 47 48 49 48
64 66 68 69 71 72 72";
		assert_eq!("0", process(input)?);
		Ok(())
	}
}
