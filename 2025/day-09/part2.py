import sys
import itertools

if len(sys.argv) > 1:
	INPUT = sys.argv[1]
else:
	INPUT = "input2.txt"


# valid max are only within lines l/r for x or above/below for y


def keyfn_by_area(tuple_p_q_area):
	return tuple_p_q_area[2]


def in_bounds(p, q, line):
	start, end = line
	left = max(p[0], q[0]) <= min(start[0], end[0])
	right = max(start[0], end[0]) <= min(p[0], q[0])
	above = max(p[1], q[1]) <= min(start[1], end[1])
	below = max(start[1], end[1]) <= min(p[1], q[1])

	within_bounds = left | right | below | above
	return within_bounds


with open(INPUT) as f:
	positions = [tuple(int(axis) for axis in line.split(",")) for line in f.readlines()]
	lines = [
		(p, q)
		for p, q in itertools.combinations(positions, 2)
		if p[0] == q[0] or p[1] == q[1]
	]
	possible_areas = [
		(p, q, ((abs(p[0] - q[0]) + 1) * (abs(p[1] - q[1]) + 1)))
		for p, q in itertools.combinations(positions, 2)
	]
	allowed_areas = []
	for p, q, area in sorted(possible_areas, key=keyfn_by_area, reverse=True):
		if all(in_bounds(p, q, line) for line in lines):
			print(area)
			break
