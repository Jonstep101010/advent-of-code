import sys

if len(sys.argv) > 1:
	INPUT = sys.argv[1]
else:
	INPUT = "input1.txt"

with open(INPUT) as f:
	ranges = f.read().split(",")
	ranges_nums = []
	for cur_range in ranges:
		rangesplit = cur_range.split("-")
		cur_range = [rangesplit[0], rangesplit[1]]
		ranges_nums.append(range(int(rangesplit[0]), int(rangesplit[1]) + 1))
	invalid_ids = 0
	for num_range in ranges_nums:
		for i in num_range:
			numstr = str(i)
			mid = int(len(numstr) / 2)
			left, right = numstr[0:mid], numstr[mid:]
			if left == right:
				print(numstr[0:mid], numstr[mid:])
				invalid_ids += i

print(invalid_ids)
