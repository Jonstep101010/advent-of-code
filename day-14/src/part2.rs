use itertools::Itertools;

#[cfg(test)]
const WIDTH: usize = 11;
#[cfg(test)]
const HEIGHT: usize = 7;

#[cfg(not(test))]
const WIDTH: usize = 101;
#[cfg(not(test))]
const HEIGHT: usize = 103;

#[derive(Debug, Clone, Copy)]
struct Robot {
	pos: (i32, i32),
	vel: (i32, i32),
}

fn print_grid(grid: &[Vec<usize>]) {
	for rows in grid.iter() {
		println!("{:?}", rows);
	}
}

fn only_unique_grid(grid: &[Vec<usize>]) -> bool {
	for row in grid.iter() {
		for &cell in row.iter() {
			if cell > 1 {
				return false;
			}
		}
	}
	true
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let mut robots: Vec<Robot> = Vec::new();
	for line in input.lines() {
		let (p, v) = line.trim().split(' ').collect_tuple().unwrap();
		let (p, v) = (p[2..].to_string(), v[2..].to_string());
		let (px, py) = p.split(',').collect_tuple().unwrap();
		let (vx, vy) = v.split(',').collect_tuple().unwrap();
		robots.push(Robot {
			pos: (px.parse().unwrap(), py.parse().unwrap()),
			vel: (vx.parse().unwrap(), vy.parse().unwrap()),
		});
	}
	// const AREA: usize = WIDTH * HEIGHT;

	let mut grid: Vec<Vec<usize>> = vec![vec![0; WIDTH]; HEIGHT];
	for Robot { pos, .. } in &robots {
		grid[pos.1 as usize][pos.0 as usize] = 1;
	}
	print_grid(&grid);
	println!(" -  - - -- - - -- --  -");
	let mut new_robots: Vec<Robot> = Vec::new();
	for robot in &robots {
		new_robots.push(Robot {
			pos: (
				// fuck, modulo in python was so much easier...
				((robot.pos.0 + (robot.vel.0 * 100)).rem_euclid(WIDTH as i32)),
				((robot.pos.1 + (robot.vel.1 * 100)).rem_euclid(HEIGHT as i32)),
			),
			vel: robot.vel,
		});
	}
	// just so we don't accidentally shoot ourselves (again) ;(((((((())))))))
	drop(robots);
	let mut grid: Vec<Vec<usize>> = vec![vec![0; WIDTH]; HEIGHT];
	for robot in &new_robots {
		grid[robot.pos.1 as usize][robot.pos.0 as usize] += 1;
	}

	print_grid(&grid);
	let (mut lt, mut rt, mut lb, mut rb) = (0, 0, 0, 0);
	const MH: i32 = (WIDTH / 2) as i32;
	const MV: i32 = (HEIGHT / 2) as i32;
	for robot in &new_robots {
		let (x, y) = (robot.pos.0, robot.pos.1);
		use std::cmp::Ordering::*;

		match x.cmp(&MH) {
			Less => match y.cmp(&MV) {
				Less => {
					lb += 1;
				}
				Greater => {
					lt += 1;
				}
				Equal => {}
			},
			Greater => match y.cmp(&MV) {
				Less => {
					rt += 1;
				}
				Greater => {
					rb += 1;
				}
				Equal => {}
			},
			_ => {}
		}
	}
	let safety_factor = rt * lt * rb * lb;
	Ok(safety_factor.to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
		assert_eq!("12", process(input)?);
		Ok(())
	}
}
