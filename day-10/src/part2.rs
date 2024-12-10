use inline_python::{Context, python};
pub fn process(input: &str) -> miette::Result<String> {
	let trailhead_scores: Context = python! {
		// # a grid consists of x and y coordinates
		// # filled with numbers from 0 to 9
		// # for each peak (9), find all unique trailheads (1)
		// # a peak has to be connected to a trailhead by trail
		// # trail (always increasing numbers by 1) can only go up, down, left, right
		// # a trailhead can be connected to multiple peaks
		// # a peak can be connected to multiple trailheads
		from copy import deepcopy

		grid = []
		trailheads = {}
		input = 'input;
		print (input)
		// # parse grid
		for line in (input.splitlines()):
			row = [int(char) for char in line]
			grid.append(row)

		// # find trailheads
		for i in range(0, len(grid)):
			for j in range(0, len(grid[i])):
				x = grid[i][j]
				if x == 0:
					trailheads[(i, j)] = []

		print("trailheads")
		print(trailheads)
		// # find paths for each trailhead
		def find_path_rec(grid, position, altitude):
			// # if not in bounds
			if not (0 <= position[0] < len(grid) and 0 <= position[1] < len(grid[0])):
				return 0
			// # if not connected/path blocked
			if altitude != grid[position[0]][position[1]]:
				return 0
			if grid[position[0]][position[1]] == 9:
				return 1
			peak_count = find_path_rec(grid, (position[0] + 1, position[1]), altitude + 1)
			peak_count += find_path_rec(grid, (position[0] - 1, position[1]), altitude + 1)
			peak_count += find_path_rec(grid, (position[0], position[1] + 1), altitude + 1)
			peak_count += find_path_rec(grid, (position[0], position[1] - 1), altitude + 1)
			return peak_count

		unique_paths = 0
		for trailhead in trailheads:
			print("trailhead:", trailhead)
			// # print(grid[trailhead[0]][trailhead[1]])
			unique_paths += find_path_rec(deepcopy(grid), deepcopy(trailhead), 0)
	};

	Ok(trailhead_scores.get::<i32>("unique_paths").to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
		assert_eq!("81", process(input)?);
		Ok(())
	}
}
