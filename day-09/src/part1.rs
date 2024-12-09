fn get_checksum(compressed_disk: &Vec<i64>) -> i64 {
	compressed_disk
		.iter()
		.enumerate()
		.map(|(i, &value)| i as i64 * value)
		.sum()
}

fn parse_disk(input: &str) -> (Vec<i64>, Vec<usize>) {
	let mut disk: Vec<i64> = Vec::new();
	let mut file_id: i64 = 0;
	for (i, c) in input.chars().enumerate() {
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
	for &i in &free_spaces {
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
	let compressed_disk = disk;
	compressed_disk
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let (mut disk, free_spaces) = parse_disk(input);
	let compressed_disk = compress(&mut disk, free_spaces);
	let checksum: i64 = get_checksum(&compressed_disk);
	Ok(checksum.to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		assert_eq!("16", process("14113")?);
		assert_eq!("6", process("133")?);
		assert_eq!("5", process("252")?);
		assert_eq!("1928", process("2333133121414131402")?);
		assert_eq!("12", process("111000000000000000001")?);
		assert_eq!("60", process("12345")?);
		Ok(())
	}
	// #[test]
	// fn test_parse() -> miette::Result<()> {
	// 	assert_eq!(
	// 		"00...111...2...333.44.5555.6666.777.888899".to_string(),
	// 		parse_block("2333133121414131402")
	// 	);
	// 	assert_eq!("0..111....22222".to_string(), parse_block("12345"));
	// 	assert_eq!("0.110".to_string(), parse_block("111000000000000000001"));
	// 	Ok(())
	// }
	// #[test]
	// fn test_compress() -> miette::Result<()> {
	// 	assert_eq!(
	// 		"0099811188827773336446555566..............".to_string(),
	// 		compress("00...111...2...333.44.5555.6666.777.888899")
	// 	);
	// 	assert_eq!(
	// 		"022111222......".to_string(),
	// 		compress(&"0..111....22222".to_string())
	// 	);
	// 	assert_eq!("0101.".to_string(), compress("0.110"));
	// 	Ok(())
	// }
	// #[test]
	// fn test_checksum() -> miette::Result<()> {
	// 	let input = "0099811188827773336446555566..............";
	// 	assert_eq!("1928", get_checksum(input).to_string());
	// 	assert_eq!("69", get_checksum("022111222......").to_string());
	// 	Ok(())
	// }
}
