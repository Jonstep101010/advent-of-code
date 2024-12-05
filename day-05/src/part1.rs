use std::collections::HashSet;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let rules = vec![];
	let rules_set = HashSet::new();
	let updates = vec![]
    for line in input.lines() {

	}

	for line in linesvec {

	}
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
