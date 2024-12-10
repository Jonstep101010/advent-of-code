fn get_checksum(compressed_disk: &[i64]) -> i64 {
	compressed_disk
		.iter()
		.enumerate()
		.map(|(i, &value)| i as i64 * value)
		.sum()
}

fn parse_disk(input: &str) -> (Vec<i64>, Vec<usize>) {
	let mut disk: Vec<i64> = Vec::new();
	let mut file_id: i64 = 0;
	for (i, c) in input.trim().chars().enumerate() {
		let x = c.to_digit(10).expect("Invalid digit") as usize;
		if i % 2 == 0 {
			// Append `x` copies of `fid` to `disk`
			disk.extend(std::iter::repeat(file_id).take(x));
			file_id += 1;
		} else {
			// Append `x` copies of `-1` to `disk`
			disk.extend(std::iter::repeat(-1).take(x));
		}
	}
	let free_spaces: Vec<usize> = disk
		.iter()
		.enumerate()
		.filter(|&(_, &value)| value == -1)
		.map(|(index, _)| index)
		.collect();
	(disk, free_spaces)
}

fn compress(disk: &mut Vec<i64>, free_spaces: Vec<usize>) -> &Vec<i64> {
	// For each free index, replace with the last non-`-1` element
	for i in free_spaces {
		// Remove trailing `-1`s
		while disk.last() == Some(&-1) {
			disk.pop();
		}
		if disk.len() <= i {
			break;
		}
		// Replace the free space with last element
		disk[i] = disk.pop().expect("Disk is empty");
	}
	disk
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let (mut disk, free_spaces) = parse_disk(input);
	let compressed_disk = compress(&mut disk, free_spaces);
	let checksum: i64 = get_checksum(compressed_disk);
	Ok(checksum.to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		// assert_eq!("16", process("14113")?);
		// assert_eq!("6", process("133")?);
		// assert_eq!("5", process("252")?);
		assert_eq!("2858", process("2333133121414131402")?);
		Ok(())
	}
}
