import sys

if len(sys.argv) > 1:
	INPUT = sys.argv[1]
else:
	INPUT = "input2.txt"

with open(INPUT) as f:
	rangeset = set()
	while True:
		line = f.readline()
		if line == "\n":
			break
		split = [int(x) for x in line.split("-")]
		start, end = split[0], split[1]
		for n in range(start, end + 1):
			rangeset.add(n)
	print(len(rangeset))
