import sys
import itertools

if len(sys.argv) > 1:
	INPUT = sys.argv[1]
else:
	INPUT = "input2.txt"


def keyfn_by_area(tuple_p_q_area):
	return tuple_p_q_area[2]


with open(INPUT) as f:
	positions = [tuple(int(axis) for axis in line.split(",")) for line in f.readlines()]
	lines = [
		(p, q)
		for p, q in itertools.combinations(positions, 2)
		if p[0] == q[0] or p[1] == q[1]
	]
	possible_areas = sorted(
		[
			(p, q, ((abs(p[0] - q[0]) + 1) * (abs(p[1] - q[1]) + 1)))
			for p, q in itertools.combinations(positions, 2)
		],
		key=keyfn_by_area,
		reverse=True,
	)

	for p, q, size in possible_areas:

		def out_of_bounds(p, q, start, end):
			"""validate rectangle (p and q) is not within line (start and end)"""
			x_left = max(p[0], q[0]) <= min(start[0], end[0])
			x_right = min(p[0], q[0]) >= max(start[0], end[0])
			y_below = max(p[1], q[1]) <= min(start[1], end[1])
			y_above = min(p[1], q[1]) >= max(start[1], end[1])

			return x_left | x_right | y_below | y_above

		if all(out_of_bounds(p, q, start, end) for start, end in lines):
			print(size)
			break
