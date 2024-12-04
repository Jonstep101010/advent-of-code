// check if the word exists starting from (x, y) in a given direction (dx, dy)
fn search(grid: &[Vec<char>], x: usize, y: usize, dx: isize, dy: isize) -> usize {
	let mut x = x as isize;
	let mut y = y as isize;
	let rows = grid.len() as isize;
	let cols = grid[0].len() as isize; // has to be square
	for ch in "XMAS".chars() {
		// Check boundaries
		if x < 0 || x >= rows || y < 0 || y >= cols {
			return 0;
		}
		// Check if the character matches
		if grid[x as usize][y as usize] != ch {
			return 0;
		}
		// Move to the next cell in the direction (dx, dy)
		x += dx;
		y += dy;
	}
	1
}

pub fn process(input: &str) -> miette::Result<String> {
	// 2d char grid
	let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

	let rows = grid.len();
	let cols = if rows > 0 { grid[0].len() } else { 0 };

	let mut count = 0;
	// Iterate over each cell in the grid
	for i in 0..rows {
		for ii in 0..cols {
			// For each direction, try to find the word starting from (i, ii)
			for &(dx, dy) in &[
				(-1, 0),  // Up
				(0, 1),   // Right
				(1, 0),   // Down
				(0, -1),  // Left
				(1, 1),   // Diag: Down-right
				(1, -1),  // Diag: Down-left
				(-1, -1), // Diag: Up-left
				(-1, 1),  // Diag: Up-right
			] {
				count += search(&grid, i, ii, dx, dy);
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
		let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
		assert_eq!("18", process(input)?);
		Ok(())
	}
}
