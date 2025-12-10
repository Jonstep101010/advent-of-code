// if a pattern matches at position (x, y)
fn match_pattern(grid: &[Vec<char>], pattern: &[Vec<char>], x: usize, y: usize) -> bool {
	for (x_i, pattern_row) in pattern.iter().enumerate() {
		for (y_i, &p_char) in pattern_row.iter().enumerate() {
			let grid_x = x + x_i;
			let grid_y = y + y_i;
			// Check boundaries
			if grid_x >= grid.len() || grid_y >= grid[0].len() {
				return false;
			}
			if p_char != '_' && grid[grid_x][grid_y] != p_char {
				return false;
			}
		}
	}
	true
}

pub fn process(input: &str) -> miette::Result<String> {
	// 2d char grid
	let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

	// 4 possible combinations to look for
	#[rustfmt::skip]
	let patterns = vec![
		vec![
			vec!['M', '_', 'S'],
			vec!['_', 'A', '_'],
			vec!['M', '_', 'S'],
		],
		vec![
			vec!['S', '_', 'M'],
			vec!['_', 'A', '_'],
			vec!['S', '_', 'M'],
		],
		vec![
			vec!['S', '_', 'S'],
			vec!['_', 'A', '_'],
			vec!['M', '_', 'M'],
		],
		vec![
			vec!['M', '_', 'M'],
			vec!['_', 'A', '_'],
			vec!['S', '_', 'S'],
		],
	];
	let rows = grid.len();
	let cols = if rows > 0 { grid[0].len() } else { 0 };

	let mut count = 0;

	// Iterate over each position where the pattern can fit
	for i in 0..=rows.saturating_sub(3) {
		for ii in 0..=cols.saturating_sub(3) {
			for pattern in &patterns {
				if match_pattern(&grid, pattern, i, ii) {
					count += 1;
				}
			}
		}
	}

	Ok(count.to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		let input = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
		assert_eq!("9", process(input)?);
		Ok(())
	}
}
