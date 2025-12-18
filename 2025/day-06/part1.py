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
			break
		else:
			calcs.extend([int(x) for x in split])
		line = f.readline().strip()
	ops = split
	# n_vecs = len(ops)
	print(ops)
	print(calcs)
	mcalc = []
	total = 0
	for i, op in enumerate(ops):
		op = mul if op == "*" else add
		mcalc.append([calcs[i]])
		for ii in range(len(ops) + i, len(calcs), len(ops)):
			mcalc[i].append(calcs[ii])
		# print(mcalc[i])
		# print(" -  -  - ")
		res = mcalc[i][0]
		for n in mcalc[i][1:]:
			# print(n)
			res = op(res, n)
		# print(res)
		total += res
print(total)


# expected: 33210 + 490 + 4243455 + 401 = 4277556
