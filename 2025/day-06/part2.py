import sys
from math import prod

if len(sys.argv) > 1:
	INPUT = sys.argv[1]
else:
	INPUT = "input2.txt"
errored = []
with open(INPUT) as f:
	# stupid ass shit WITH spaces
	lines = [line.strip("\n") for line in f]
	max_len = max(len(line) for line in lines)

	# padding to ensure safe access
	rows = [line.ljust(max_len) for line in lines]

	ops = list(rows.pop())

	total = 0
	newdigits = []
	alldigits = []
	for col in reversed(range(len(ops))):
		digits = ""
		for row in rows:
			digit = row[col]
			if digit == " ":
				digits += ""
			else:
				digits += digit
		if digits == "":
			newdigits = []
		else:
			newdigits.append(int(digits))
		op = ops[col]
		if op == "*":
			total += prod(newdigits)
			alldigits.append(newdigits)
		elif op == "+":
			total += sum(newdigits)
			alldigits.append(newdigits)
		elif op == " ":
			pass

if not (
	[356, 24, 1] in alldigits
	and [4, 431, 623] in alldigits
	and [175, 581, 32] in alldigits
	and [8, 248, 369] in alldigits
):
	print(f"error in {alldigits}")
# expected: 1058 + 3253600 + 625 + 8544 = 3263827
print(total)
