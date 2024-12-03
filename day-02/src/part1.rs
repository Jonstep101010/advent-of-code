use std::{cmp::Ordering, collections::HashSet};

use itertools::Itertools;
use miette::Error;

// check single report
pub fn check_safe(report: &Vec<i32>) -> miette::Result<String> {
	if report.len() < 2 {
		return Err(Error::msg("too short"));
	}
	let mut seen = HashSet::new();
	for &value in report {
		if !seen.insert(value) {
			return Err(Error::msg("duplicates"));
		}
	}
	// check first two levels for order
	let order = {
		if report[0] < report[1] {
			Ordering::Greater
		} else {
			Ordering::Less
		}
	};

	for (a, b) in report.iter().tuple_windows() {
		let diff = a.abs_diff(*b);
		if diff == 0 {
			return Err(Error::msg("duplicates"));
		} else if diff > 3 {
			// check distance between the two numbers: <= 3
			return Err(Error::msg("diff > 3"));
		} else {
			// check that order has been kept
			if a < b && order != Ordering::Greater {
				return Err(Error::msg("order has changed to descending"));
			} else if a > b && order != Ordering::Less {
				return Err(Error::msg("order has changed to ascending"));
			}
		}
	}
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
		match check_safe(report) {
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
