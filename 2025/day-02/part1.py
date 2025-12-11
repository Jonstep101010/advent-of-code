import sys

if len(sys.argv) > 1:
	INPUT = sys.argv[1]
else:
	INPUT = "input1.txt"

invalid_ids = 0
with open(INPUT) as f:
	ranges = f.read().split(",")
	for cur_range in ranges:
		rangesplit = cur_range.split("-")
		for i in range(int(rangesplit[0]), int(rangesplit[1]) + 1):
			numstr = str(i)
			mid = int(len(numstr) / 2)
			left, right = numstr[:mid], numstr[mid:]
			if left == right:
				print(numstr[:mid], numstr[mid:])
				invalid_ids += i

print(invalid_ids)
