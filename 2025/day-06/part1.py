import sys
from operator import mul, add

if len(sys.argv) > 1:
	INPUT = sys.argv[1]
else:
	INPUT = "input1.txt"

with open(INPUT) as f:
	line = f.readline().strip()
	calcs = []
	ops = None
	while True:
		split = line.split()
		if "+" in line or "*" in line or "-" in line or "/" in line:
			mcalc = []
			total = 0
			for i, op in enumerate(split):
				op = mul if op == "*" else add
				cur = [calcs[ii] for ii in range(i, len(calcs), len(split))]
				res = cur[0]
				for n in cur[1:]:
					res = op(res, n)
				total += res
			print(total)
			break
		else:
			calcs.extend([int(x) for x in split])
			line = f.readline().strip()

# expected: 33210 + 490 + 4243455 + 401 = 4277556
