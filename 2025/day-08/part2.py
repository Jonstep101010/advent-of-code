import sys

if len(sys.argv) > 1:
	INPUT = sys.argv[1]
else:
	INPUT = "input2.txt"


class UnionFind:
	def __init__(self):
		self.parent = {}
		self.rank = {}
		self.size = {}

	def add(self, x):
		"""Add a new element to the union-find structure."""
		if x not in self.parent:
			self.parent[x] = x
			self.rank[x] = 0
			self.size[x] = 1

	def __find(self, x):
		"""Find the root of x with path compression."""
		if self.parent[x] != x:
			self.parent[x] = self.__find(self.parent[x])
		return self.parent[x]

	def union(self, x, y) -> bool:
		"""Union two sets by rank, updating sizes. Returns True if they were merged."""
		root_x = self.__find(x)
		root_y = self.__find(y)

		if root_x == root_y:
			return False

		if self.rank[root_x] < self.rank[root_y]:
			self.parent[root_x] = root_y
			self.size[root_y] += self.size[root_x]
		elif self.rank[root_x] > self.rank[root_y]:
			self.parent[root_y] = root_x
			self.size[root_x] += self.size[root_y]
		else:
			self.parent[root_y] = root_x
			self.size[root_x] += self.size[root_y]
			self.rank[root_x] += 1
		return True

	def get_size(self, x) -> int:
		"""Get the size of the set containing x."""
		return self.size[self.__find(x)]


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
			all_pairs.append((p, q))
	all_pairs.sort(
		key=lambda pair: sum(
			(axis[0] - axis[1]) ** 2 for axis in zip(pair[0][:3], pair[1][:3])
		)
	)

	total_points = len(pos)
	for p, q in all_pairs:
		if uf.union(p, q):
			if uf.get_size(p) == total_points:
				print(p[0] * q[0])
				break
