import sys

if len(sys.argv) > 1:
	INPUT = sys.argv[1]
else:
	INPUT = "input2.txt"

with open(INPUT) as f:
	ranges = []
	while True:
		line = f.readline()
		if not line or line == "\n":
			break
		split = [int(x) for x in line.split("-")]
		ranges.append((split[0], split[1]))

	ranges.sort()

	count = 0
	start, end = ranges[0]
	for range_start, range_end in ranges[1:]:
		if range_start > end + 1:
			count += end - start + 1
			start, end = range_start, range_end
		end = max(end, range_end)

	count += end - start + 1
	print(count)
