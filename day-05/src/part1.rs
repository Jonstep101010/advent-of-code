use itertools::Itertools;

// check single update
// return Ok(mid_val) if safe, Err(()) if not
// for example test caste:
// 75 is correctly first because there are rules that put each other page after it: 75|47, 75|61, 75|53, and 75|29.
// 47 is correctly second because 75 must be before it (75|47) and every other page must be after it according to 47|61, 47|53, and 47|29.
// 61 is correctly in the middle because 75 and 47 are before it (75|61 and 47|61) and 53 and 29 are after it (61|53 and 61|29).
// 53 is correctly fourth because it is before page number 29 (53|29).
// 29 is the only page left and so is correctly last.
fn check_safe(rules: &Vec<(u32, u32)>, update: &Vec<u32>) -> Result<u32, ()> {
	for numbers in update.iter().tuple_windows::<(_, _)>() {
		let mut found = false;
		for rule in rules {
			if rule.0 == *numbers.0 && rule.1 == *numbers.1 {
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

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let mut rules = vec![];
	let mut updates = vec![];
	for line in input.lines() {
		if line.contains("|") {
			let mut parts = line.split("|");
			let left = parts.next().unwrap().trim().parse::<u32>().unwrap();
			let right = parts.next().unwrap().trim().parse::<u32>().unwrap();
			rules.push((left, right));
		} else if line.contains(",") {
			let parts = line.split(",");
			updates.push(
				parts
					.map(|x| x.trim().parse::<u32>().unwrap())
					.collect_vec(),
			);
		}
	}
	// dbg!(&rules);
	// dbg!(&updates);
	let mut middle_pages_sum = 0;
	for update in updates {
		// dbg!(&update);
		match check_safe(&rules, &update) {
			Ok(mid_val) => {
				middle_pages_sum += mid_val;
			}
			Err(_) => {
				// do nothing
			}
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
