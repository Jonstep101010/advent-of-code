# a grid consistst of x and y coordinates
# filled with numbers from 0 to 9
# for each peak (9), find all unique trailheads (1)
# a peak has to be connected to a trailhead by trail
# trail (always increasing numbers by 1) can only go up, down, left, right
# a trailhead can be connected to multiple peaks
# a peak can be connected to multiple trailheads
# per peak to trailhead connection, there can only be one trail
from copy import deepcopy

grid = []
peaks = {}
trailheads = {}
input = open('input1.txt').read()
print (input)
# parse grid
for line in (input.splitlines()):
	row = [int(char) for char in line]
	grid.append(row)

# find peaks, trailheads
for i in range(0, len(grid)):
	for j in range(0, len(grid[i])):
		x = grid[i][j]
		if x == 9:
			peaks[(i, j)] = []
		if x == 0:
			trailheads[(i, j)] = []

print("trailheads")
print(trailheads)
print("peaks")
print(peaks)
# find paths for each trailhead
def find_path_rec(grid, position, altitude):
	# if not in bounds
	if not (0 <= position[0] < len(grid) and 0 <= position[1] < len(grid[0])):
		return 0
	# if not connected/path blocked
	if altitude != grid[position[0]][position[1]]:
		return 0
	if grid[position[0]][position[1]] == 9:
		grid[position[0]][position[1]] = 0
		return 1
	peak_count = find_path_rec(grid, (position[0] + 1, position[1]), altitude + 1)
	peak_count += find_path_rec(grid, (position[0] - 1, position[1]), altitude + 1)
	peak_count += find_path_rec(grid, (position[0], position[1] + 1), altitude + 1)
	peak_count += find_path_rec(grid, (position[0], position[1] - 1), altitude + 1)
	return peak_count

unique_paths = 0
for trailhead in trailheads:
	print("trailhead:", trailhead)
	# print(grid[trailhead[0]][trailhead[1]])
	unique_paths += find_path_rec(deepcopy(grid), deepcopy(trailhead), 0)
print(unique_paths)