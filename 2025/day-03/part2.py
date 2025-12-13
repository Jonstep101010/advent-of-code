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
		bank_digits = []
		joltage_output = [None] * 12
		largest_idx = None
		for c in battery_bank:
			if largest_idx is None or int(c) > bank_digits[largest_idx]:
				largest_idx = len(bank_digits)
			bank_digits.append(int(c))
		offset = 0
		for i in range(0, 12):
			current_slice = bank_digits[offset : (len(bank_digits) - 11 + i)]
			relative_index, first_max = max(
				enumerate(current_slice), key=lambda x: x[1]
			)
			joltage_output[i] = first_max
			offset = offset + relative_index + 1
		assert None not in joltage_output
		jolts.append(int("".join(str(n) for n in joltage_output)))
		# output.write(str(largest_joined) + "\n")
# print(jolts)
# output.close()
# print("- - TOTAL - -")
print(sum(jolts))
# assert sum(jolts) == 3121910778619

# 987654321111 + 811111111119 + 434234234278 + 888911112111
#  = 3121910778619
