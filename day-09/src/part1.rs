/*
get_checksum:
add up the result of multiplying each of these compressed blocks' position with the file ID number it contains.
The leftmost block is in position 0. If a block contains free space, skip it instead

get_checksum("022111222......") == 69
get_checksum("0099811188827773336446555566..............") == 1928
*/
fn get_checksum(compressed: &str) -> usize {
	let mut checksum = 0;
	for (i, val) in compressed.chars().enumerate() {
		if val.is_ascii_digit() {
			checksum += i * (val.to_digit(10).unwrap() as usize);
		}
	}
	checksum
}

fn parse_block(input: &str) -> String {
	let mut block = String::new();
	for (idx, val_idx) in input.chars().enumerate() {
		// dbg!(val_idx);
		let val = val_idx.to_digit(10);
		if val.is_none() {
			break;
		}
		let val = val.unwrap();
		let sequence = if idx % 2 == 0 {
			// number
			let mut num_seq = String::new();
			let to_push = if idx == 0 {
				"0".to_string()
			} else {
				(idx / 2).to_string()
			};
			for _ in 1..=val {
				num_seq.push_str(&to_push);
			}
			num_seq
		} else {
			// dots
			let mut num_seq = String::new();
			for _ in 1..=val {
				num_seq.push('.');
			}
			num_seq
		};
		block.push_str(sequence.as_str());
	}
	block
}

#[must_use]
pub fn compress(block: &str) -> String {
	let mut compressed = block.to_owned();
	loop {
		// find position of first '.', make sure it is before last digit
		let first_free_idx = compressed.find('.');
		let last_num = compressed.rfind(|c: char| c.is_numeric());
		if first_free_idx.is_none() || first_free_idx >= last_num || last_num.is_none() {
			break;
		}
		// swap contents
		let mut ptrs = compressed.into_bytes();
		let tmp = ptrs[last_num.unwrap()];
		ptrs[last_num.unwrap()] = ptrs[first_free_idx.unwrap()];
		ptrs[first_free_idx.unwrap()] = tmp;
		compressed = String::from_utf8(ptrs).unwrap();
	}
	compressed
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	// @audit only certain that example parsing is correct
	dbg!(&input);
	let block = parse_block(input);
	dbg!(&block);
	let compressed = compress(&block);
	dbg!(&compressed);
	let checksum = get_checksum(&compressed);
	Ok(checksum.to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		assert_eq!("1928", process("2333133121414131402")?);
		assert_eq!("69", process("12345")?);
		Ok(())
	}
	#[test]
	fn test_parse() -> miette::Result<()> {
		assert_eq!(
			"00...111...2...333.44.5555.6666.777.888899".to_string(),
			parse_block("2333133121414131402")
		);
		assert_eq!("0..111....22222".to_string(), parse_block("12345"));
		Ok(())
	}
	#[test]
	fn test_compress() -> miette::Result<()> {
		assert_eq!(
			"0099811188827773336446555566..............".to_string(),
			compress("00...111...2...333.44.5555.6666.777.888899")
		);
		assert_eq!(
			"022111222......".to_string(),
			compress(&"0..111....22222".to_string())
		);
		Ok(())
	}
	#[test]
	fn test_checksum() -> miette::Result<()> {
		let input = "0099811188827773336446555566..............";
		assert_eq!("1928", get_checksum(input).to_string());
		assert_eq!("69", get_checksum("022111222......").to_string());
		Ok(())
	}
}
