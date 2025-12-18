import sys
from operator import mul, add

if len(sys.argv) > 1:
	INPUT = sys.argv[1]
else:
	INPUT = "input2.txt"

with open(INPUT) as f:
	line = f.readline().strip()
	calcs = []
	ops = None
	while True:
		split = line.split()
		if "+" in line or "*" in line or "-" in line or "/" in line:
			break
		else:
			calcs.extend([x for x in split])
		line = f.readline().strip()
	ops = split
	# n_vecs = len(ops)
	# print(ops)
	# print(calcs)
	mcalc = []
	total = 0
	for i, op in enumerate(ops):
		op = mul if op == "*" else add
		mcalc.append([calcs[i]])
		for ii in range(len(ops) + i, len(calcs), len(ops)):
			mcalc[i].append(calcs[ii])
		# print(mcalc[i])
		# print(" -  -  - ")
		ii = 0
		cur = []
		for nstr in mcalc[i]:
			arr = [x[-1] for x in reversed(nstr)]
			for x in nstr:
				ii += 1
			cur.append((arr))
		print(cur)

		for x in [max([len(c) for c in cur])]:
			print(x)
			# ([['3', '2', '1'], ['5', '4'], ['6']],)
			# -> longest len first: 3
			# -> continue from top to bottom on lower lenghts: 5, 6
			# ['2', '1'], ['4'] -> 2, 4
			# ['1']
			assert False

		# res = 356 * 24 * 1 = 8544
		# print(res)
		# total += res
print(total)


# expected: 1058 + 3253600 + 625 + 8544 = 3263827
