use std::cmp::Ordering;

use itertools::Itertools;
use miette::Error;

// check single report
pub fn check_safe(report: &[i32]) -> miette::Result<String> {
	if report.len() < 2 {
		return Err(Error::msg("too short"));
	}
	report.iter().try_for_each(|&value| {
		if report.iter().filter(|&&v| v == value).count() > 1 {
			Err(Error::msg("duplicates"))
		} else {
			Ok(())
		}
	})?;
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
		}
		// check that order has been kept
		if a < b && order != Ordering::Greater {
			return Err(Error::msg("order has changed to descending"));
		} else if a > b && order != Ordering::Less {
			return Err(Error::msg("order has changed to ascending"));
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
				let mut safe = false;
				// iterate over and try every single element until all failed/safe found
				for i in 0..report.len() {
					let mut clone = report.clone();
					clone.remove(i);
					if check_safe(&clone).is_ok() {
						safe = true;
						break;
					}
				}
				if safe {
					safe_reports += 1
				} else {
					println!("Error: {}", e);
				}
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
		todo!("haven't built test yet");
		let input = "";
		assert_eq!("", process(input)?);
		Ok(())
	}
}
