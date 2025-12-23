import sys

if len(sys.argv) > 1:
	INPUT = sys.argv[1]
else:
	INPUT = "input1.txt"

with open(INPUT) as f:
	filestr = f.readlines()
	beam_positions = set()
	for y, line in enumerate(filestr):
		if "S" in line:
			manifold_pos = line.index("S")
			beam_positions.add((manifold_pos, y + 1))
			break
	# print(filestr)
	test = 0
	for y, line in enumerate(filestr):
		for x, c in enumerate(line):
			if c == "^" and (x, y - 1) in beam_positions:
				test += 1
				beam_positions.add((x - 1, y))
				beam_positions.add((x + 1, y))
			elif (x, y - 1) in beam_positions:
				beam_positions.add((x, y))
	print(beam_positions)
	print(len(beam_positions))

	# beam_order = []
	# for pos in beam_positions:
	# 	beam_order.append(pos)
	# beam_order = sorted(beam_order)

	# print()
	# print(beam_order)
	print(test)

# for each splitter, a tachyon beam can at most be split into two
# position below the splitter cannot have beam unless there are two splitters next to each other
# total beams - (current * 2) at most and current at least
# for each beam where the next one has at least one more
