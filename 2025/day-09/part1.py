import sys
import itertools

if len(sys.argv) > 1:
	INPUT = sys.argv[1]
else:
	INPUT = "input1.txt"

with open(INPUT) as f:
	positions = [tuple(int(axis) for axis in line.split(",")) for line in f.readlines()]
	# print(pos)

	max_area = max(
		[
			(abs(p[0] - q[0]) + 1) * (abs(p[1] - q[1]) + 1)
			for p, q in itertools.combinations(positions, 2)
		]
	)
	print(max_area)
