import sys
import itertools

if len(sys.argv) > 1:
	INPUT = sys.argv[1]
else:
	INPUT = "input1.txt"

with open(INPUT) as f:
	pos = []
	for line in f.readlines():
		pos.append([int(line_it) for line_it in line.split(",")])
	print(pos)

	areas = []
	for pair in itertools.combinations(pos, 2):
		areas.append(
			(abs(pair[0][0] - pair[1][0]) + 1) * (abs(pair[0][1] - pair[1][1]) + 1)
		)
	print(max(areas))
