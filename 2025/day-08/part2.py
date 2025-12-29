import sys

if len(sys.argv) > 1:
	INPUT = sys.argv[1]
else:
	INPUT = "input2.txt"

unions_dict = {}


class Union:
	def __init__(self, members):
		self.members = members
		for m in members:
			unions_dict[m] = self

	def union(self, other):
		return Union(self.members | other.members)


with open(INPUT) as f:
	pos = []
	for line in f.readlines():
		p = tuple(int(axis) for axis in line.split(","))
		pos.append(p)
		Union({p})

	all_pairs = []
	for i, p in enumerate(pos):
		for ii in range(i + 1, len(pos)):
			q = pos[ii]
			all_pairs.append((p, q))
	all_pairs.sort(
		key=lambda pair: sum(
			(axis[0] - axis[1]) ** 2 for axis in zip(pair[0][:3], pair[1][:3])
		)
	)
	# MAX_CONNECTIONS = (
	# 	1000 if INPUT == "input2.txt" else 10
	# )
	for i, (p, q) in enumerate(all_pairs):
		# if i == MAX_CONNECTIONS:
		# 	unions = set()
		# 	for p in pos:
		# 		unions.add(unions_dict[p])
		# 	unions = list(sorted(unions, key=lambda u: len(u.members), reverse=True))
		# 	print(
		# 		len(unions[0].members) * len(unions[1].members) * len(unions[2].members)
		# 	)
		if unions_dict[p] != unions_dict[q]:
			u = unions_dict[p].union(unions_dict[q])
			if len(u.members) == len(pos):
				print(p[0] * q[0])
				break
