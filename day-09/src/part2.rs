#![warn(clippy::pedantic)]

use std::collections::HashMap;

fn get_checksum(files: &HashMap<usize, (usize, usize)>) -> usize {
	let mut checksum: usize = 0;
	for (fid, (pos, size)) in files {
		for x in *pos..(*pos + *size) {
			checksum += fid * x;
		}
	}
	checksum
}

#[allow(clippy::type_complexity)]
fn parse_files(input: &str) -> (HashMap<usize, (usize, usize)>, Vec<(usize, usize)>) {
	let mut files = HashMap::new();
	let mut free_spaces: Vec<(usize, usize)> = Vec::new();
	let mut file_id: usize = 0;
	let mut pos: usize = 0;
	for (i, c) in input.chars().enumerate() {
		let x = c.to_digit(10).unwrap() as usize;
		if i % 2 == 0 {
			assert!(x != 0, "File size cannot be 0");
			files.insert(file_id, (pos, x));
			file_id += 1;
		} else if x != 0 {
			free_spaces.push((pos, x));
		}
		pos += x;
	}
	(files, free_spaces)
}

// move in order of decreasing file_id number
fn move_files(
	file_ids: Vec<usize>,
	mut files: HashMap<usize, (usize, usize)>,
	free_spaces: &mut Vec<(usize, usize)>,
) -> HashMap<usize, (usize, usize)> {
	for file_id in file_ids {
		let (pos, size) = files.get(&file_id).copied().unwrap();
		let mut i = 0;
		while i < free_spaces.len() {
			let (fpos, fsize) = free_spaces[i];
			if fpos >= pos {
				free_spaces.truncate(i); // Remove extra free spaces
				break;
			}
			if size <= fsize {
				// Move file to free space
				files.insert(file_id, (fpos, size));
				if size == fsize {
					free_spaces.remove(i);
				} else {
					free_spaces[i] = (fpos + size, fsize - size);
				}
				break;
			}
			i += 1;
		}
	}
	files
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
	let (files, mut free_spaces) = parse_files(input.trim());
	let file_ids: Vec<usize> = {
		let mut file_id_keys: Vec<usize> = files.keys().copied().collect();
		file_id_keys.sort_unstable_by(|a, b| b.cmp(a)); // Sort in reverse order
		file_id_keys
	};

	let files = move_files(file_ids, files, &mut free_spaces);

	let checksum = get_checksum(&files);
	Ok(checksum.to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		assert_eq!("2858", process("2333133121414131402")?);
		Ok(())
	}
}
