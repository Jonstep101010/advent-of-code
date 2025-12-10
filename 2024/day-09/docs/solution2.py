files = {}
free_spaces = []
file_id = 0
pos = 0
for i, char in enumerate(open('input1.txt').read().strip()):
	x = int(char)
	if i % 2 == 0:
		if x == 0:
			raise ValueError('File size cannot be 0')
		files[file_id] = (pos, x)
		file_id += 1
	else:
		if x != 0:
			free_spaces.append((pos, x))
	pos += x

while file_id > 0:
	file_id -= 1
	(pos, size) = files[file_id]
	for i, (fpos, fsize) in enumerate(
		free_spaces
	):  # basically enumerate pairs of free spaces
		if fpos >= pos:
			free_spaces = free_spaces[:i]  # get rid of the extra free spaces
			break
		if size <= fsize:
			files[file_id] = (fpos, size)
			if size == fsize:
				free_spaces.pop(i)
			else:
				free_spaces[i] = (fpos + size, fsize - size)
			break

# print(files)
total = 0
for file_id, (pos, size) in files.items():
	for x in range(pos, pos + size):
		total += file_id * x
print(total)
