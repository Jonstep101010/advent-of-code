import sys
from operator import mul, add

if len(sys.argv) > 1:
	INPUT = sys.argv[1]
else:
	INPUT = "input2.txt"
errored = []
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
			arr = [int(x[-1]) for x in reversed(nstr)]
			for x in nstr:
				ii += 1
			cur.append((arr))
		print(cur)

		newdigits = []
		# cur = [[4, 6], [2, 3], [4, 1, 3]]
		newd_pos = 0
		while True:
			# print(digits)
			try:
				max_index = max(range(len(cur)), key=lambda idx: len(cur[idx]))
			except ValueError:
				break
			ii = 0
			maxnum = ""
			while cur != []:
				if [] in cur:
					cur.remove([])
				print(f"cur {cur}")
				try:
					print(f"maxnum: {cur[max_index][ii]}")
					maxnum += str(cur[max_index][ii])
					print(f"max: {cur[max_index]}")
					cur[max_index] = cur[max_index][1:]
				except IndexError:
					print(f"idxerr: {cur}")
					cur = []
					break
				if max_index == len(cur) - 1:
					print(f"appending to array {int(maxnum)}")
					newdigits.append(int(maxnum))
					newd_pos += 1
					maxnum = ""
					max_index = 0
					# break
				elif cur[max_index] == []:
					continue
				else:
					print(f"incrementing on {cur} with maxindex = {max_index}")
					max_index += 1
			print(f"newds = {newdigits}")
			if not (
				newdigits == [356, 24, 1]
				or newdigits == [4, 431, 623]
				or newdigits == [175, 581, 32]
				or newdigits == [8, 248, 369]
			):
				print(f"error in {newdigits}")
				errored.append(newdigits)
			# ([['3', '2', '1'], ['5', '4'], ['6']],)
			# -> longest len first: 3
			# -> continue from top to bottom on lower lenghts: 5, 6
			# ['2', '1'], ['4'] -> 2, 4
			# ['1']

		# res = 356 * 24 * 1 = 8544
		# print(res)
		# total += res
if errored != []:
	print("error in these")
	for e in errored:
		print(e)
print(total)


# expected: 1058 + 3253600 + 625 + 8544 = 3263827
