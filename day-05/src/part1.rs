#![warn(clippy::pedantic)]

use itertools::Itertools;

// check single update
// return Ok(mid_val) if safe, Err(()) if not
// for example test caste:
// 75 is correctly first because there are rules that put each other page after it: 75|47, 75|61, 75|53, and 75|29.
// 47 is correctly second because 75 must be before it (75|47) and every other page must be after it according to 47|61, 47|53, and 47|29.
// 61 is correctly in the middle because 75 and 47 are before it (75|61 and 47|61) and 53 and 29 are after it (61|53 and 61|29).
// 53 is correctly fourth because it is before page number 29 (53|29).
// 29 is the only page left and so is correctly last.
fn check_safe(rules: &Vec<(u32, u32)>, update: &[u32]) -> Result<u32, ()> {
	for (&left, &right) in update.iter().tuple_windows::<(_, _)>() {
		let mut found = false;
		for rule in rules {
			if rule.0 == left && rule.1 == right {
				found = true;
				break;
			}
		}
		if !found {
			return Err(());
		}
	}
	// dbg!(update[update.len() / 2]);
	Ok(update[update.len() / 2])
}

fn parse(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
	let mut rules = vec![];
	let mut updates = vec![];
	for line in input.lines() {
		if line.contains('|') {
			let (left, right) = line
				.split_once('|')
				.map(|(l, r)| {
					(
						l.trim().parse::<u32>().unwrap(),
						r.trim().parse::<u32>().unwrap(),
					)
				})
				.unwrap();
			rules.push((left, right));
		} else if line.contains(',') {
			updates.push(
				line.split(',')
					.map(|x| x.trim().parse::<u32>().unwrap())
					.collect_vec(),
			);
		}
	}
	(rules, updates)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let (rules, updates) = parse(input);
	// dbg!(&rules);
	// dbg!(&updates);
	let mut middle_pages_sum = 0;
	for update in updates {
		// dbg!(&update);
		if let Ok(mid_val) = check_safe(&rules, &update) {
			middle_pages_sum += mid_val;
		}
	}
	Ok(middle_pages_sum.to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
		// valid middle page numbers: 61 + 53 + 29
		assert_eq!("143", process(input)?);
		Ok(())
	}
}
