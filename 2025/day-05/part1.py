import sys

if len(sys.argv) > 1:
	INPUT = sys.argv[1]
else:
	INPUT = "input1.txt"

with open(INPUT) as f:
	fresh_ranges = []
	while True:
		line = f.readline()
		if line == "\n":
			break
		split = [int(x) for x in line.split("-")]
		start, end = split[0], split[1]
		fresh_ranges.append(range(start, end + 1))
	# print(fresh_ranges)
	ingredients = []
	while True:
		line = f.readline()
		if line == "":
			break
		ingredients.append(int(line))
	# print(ingredients)
	# fresh_total = 0
	# for ingredient in ingredients:
	# 	for fresh in fresh_ranges:
	# 		if ingredient in fresh:
	# 			# print(ingredient)
	# 			fresh_total += 1
	# 			break
	print(
		sum(
			1
			for ingredient in ingredients
			if any(ingredient in fresh for fresh in fresh_ranges)
		)
	)
