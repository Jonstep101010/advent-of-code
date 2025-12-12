import sys

if len(sys.argv) > 1:
	INPUT = sys.argv[1]
else:
	INPUT = "input1.txt"

jolts = []
with open(INPUT) as f:
	filestr = f.read()
	battery_banks = filestr.splitlines()
	# output = open("output1jolts.txt", "w")
	for battery_bank in battery_banks:
		largest_overall = None
		second_largest = int(battery_bank[1])
		bank_digits = []
		for c in battery_bank:
			if largest_overall is None or int(c) > largest_overall:
				if largest_overall is not None:
					second_largest = largest_overall
				largest_idx = len(bank_digits)
				largest_overall = int(c)
			bank_digits.append(int(c))

		def create_digit(first_digit: int, second_digit: int) -> int:
			return int(str(first_digit) + str(second_digit))

		if largest_overall == bank_digits[0]:
			largest_joined = create_digit(largest_overall, second_largest)
		else:
			largest_joined = create_digit(second_largest, largest_overall)
		for i, second_digit in enumerate(bank_digits[largest_idx + 1 :]):
			maybe_larger = create_digit(largest_overall, second_digit)
			if maybe_larger > largest_joined:
				largest_joined = maybe_larger
		jolts.append(largest_joined)
		# print("- -  - -")
		# print(f"joltage = {largest_joined}")
		# print(bank_digits)
		# output.write(str(largest_joined) + "\n")
# print(jolts)
# output.close()
# print("- - TOTAL - -")
print(sum(jolts))
