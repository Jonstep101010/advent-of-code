import sys

if len(sys.argv) > 1:
	INPUT = sys.argv[1]
else:
	INPUT = "input2.txt"

with open(INPUT) as f:
	filestr = f.read()
	print(filestr)
