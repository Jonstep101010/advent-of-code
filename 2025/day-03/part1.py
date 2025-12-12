import sys

if len(sys.argv) > 1:
	INPUT = sys.argv[1]
else:
	INPUT = "input1.txt"

jolts = []
with open(INPUT) as f:
	filestr = f.read()
	battery_banks = filestr.splitlines()
	output = open("output1jolts.txt", "w")
	for battery_bank in battery_banks:
		first = int(battery_bank[0])
		largest_slice = []
		bank_digits = []
		largest_idx = None
		bank_digits.append(first)
		for c in battery_bank[1:]:
			if largest_slice == [] or int(c) > max(largest_slice):
				largest_slice.append(int(c))
				largest_idx = len(bank_digits)
			bank_digits.append(int(c))
		largest_overall = max(max(largest_slice), first)
		first_max = first
		second_max = largest_overall
		for i, n in enumerate(largest_slice):
			if first_max < n and n != largest_overall:
				first_max = n
				largest_slice = largest_slice[i:]

		def create_digit(largest_overall: int, n: int) -> int:
			return int(str(largest_overall) + str(n))

		largest_joined = create_digit(first_max, largest_overall)
		if largest_slice == []:
			print("lol")
		if len(largest_slice) == 1:
			print("fuck")
		if first == largest_overall:
			print("fucked")
			largest_joined = create_digit(first, bank_digits[1])
			largest_idx = 1
		else:
			largest_joined = create_digit(first_max, largest_overall)
		for i, c in enumerate(bank_digits[largest_idx + 1 :]):
			if c == largest_overall:
				largest_idx = i
				# print(bank_digits[largest_idx + 1 :])
				break
		for i, c in enumerate(bank_digits[largest_idx + 1 :]):
			# print(c)
			maybe_larger = create_digit(largest_overall, c)
			if maybe_larger > largest_joined:
				largest_joined = maybe_larger
		print("- -  - -")
		print(f"jolts = {largest_joined}")
		print(bank_digits)
		jolts.append(largest_joined)
		output.write(str(largest_joined) + "\n")
# print(jolts)
output.close()
print("- - TOTAL - -")
print(sum(jolts))
