import sys

if len(sys.argv) > 1:
	INPUT = sys.argv[1]
else:
	INPUT = "input1.txt"


class UnionFind:
	def __init__(self):
		self.parent = {}
		self.rank = {}

	def add(self, x):
		"""Add a new element to the union-find structure."""
		if x not in self.parent:
			self.parent[x] = x
			self.rank[x] = 0

	def __find(self, x):
		"""Find the root of x with path compression."""
		if self.parent[x] != x:
			self.parent[x] = self.__find(self.parent[x])
		return self.parent[x]

	def union(self, x, y) -> bool:
		"""Union two sets by rank. Returns True if they were merged."""
		root_x = self.__find(x)
		root_y = self.__find(y)

		if root_x == root_y:
			return False

		if self.rank[root_x] < self.rank[root_y]:
			self.parent[root_x] = root_y
		elif self.rank[root_x] > self.rank[root_y]:
			self.parent[root_y] = root_x
		else:
			self.parent[root_y] = root_x
			self.rank[root_x] += 1
		return True

	def get_groups(self) -> dict:
		"""Return a dict mapping roots to lists of members."""
		groups = {}
		for x in self.parent:
			root = self.__find(x)
			if root not in groups:
				groups[root] = []
			groups[root].append(x)
		return groups


with open(INPUT) as f:
	uf = UnionFind()
	pos = []
	for line in f.readlines():
		p = tuple(int(axis) for axis in line.split(","))
		pos.append(p)
		uf.add(p)

	all_pairs = []
	for i, p in enumerate(pos):
		for ii in range(i + 1, len(pos)):
			q = pos[ii]
			dist = sum((axis[0] - axis[1]) ** 2 for axis in zip(p[:3], q[:3]))
			all_pairs.append((dist, p, q))
	all_pairs.sort()
	MAX_CONNECTIONS = 1000 if INPUT == "input1.txt" else 10
	pairs_processed = 0
	for dist, p, q in all_pairs:
		uf.union(p, q)
		pairs_processed += 1
		if pairs_processed == MAX_CONNECTIONS:
			groups = uf.get_groups()
			sizes = sorted([len(members) for members in groups.values()], reverse=True)
			print(sizes[0] * sizes[1] * sizes[2])
			break
