import sys

if len(sys.argv) > 1:
	INPUT = sys.argv[1]
else:
	INPUT = "input1.txt"

with open(INPUT) as f:
	lines = f.readlines()
	roll_positions = set()
	for y, line in enumerate(lines):
		for x, c in enumerate(line):
			if c == "@":
				roll_positions.add((x, y))
	print(len(roll_positions))
	removable_rolls = 0
	while True:
		removed = set()
		for x, y in roll_positions:
			count = 0
			if (x - 1, y) in roll_positions:
				count += 1
			if (x - 1, y + 1) in roll_positions:
				count += 1
			if (x - 1, y - 1) in roll_positions:
				count += 1
			if (x, y - 1) in roll_positions:
				count += 1
			if (x, y + 1) in roll_positions:
				count += 1
			if (x + 1, y) in roll_positions:
				count += 1
			if (x + 1, y + 1) in roll_positions:
				count += 1
			if (x + 1, y - 1) in roll_positions:
				count += 1
			# not too many neighbors: can take roll
			if count < 4:
				removed.add((x, y))
		if len(removed) == 0:
			break
		removable_rolls += len(removed)
		for x, y in removed:
			if (x, y) in roll_positions:
				roll_positions.remove((x, y))
	print(removable_rolls)
