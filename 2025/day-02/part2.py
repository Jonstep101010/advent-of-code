import sys

if len(sys.argv) > 1:
	INPUT = sys.argv[1]
else:
	INPUT = "input2.txt"

invalid_ids = 0
with open(INPUT) as f:
	ranges = f.read().strip().split(",")
	for cur_range in ranges:
		rangesplit = cur_range.split("-")
		for i_outer in range(int(rangesplit[0]), int(rangesplit[1]) + 1):
			numstr = str(i_outer)
			mid = int(len(numstr) / 2)
			left, right = numstr[:mid], numstr[mid:]
			if left == right:
				invalid_ids += i_outer
				print(i_outer)
			else:
				first_seq = numstr[0]
				first_rep = None
				# last_seq = None
				maybe_seq = None
				for i in range(1, len(numstr)):
					if numstr[i] == first_seq:
						# print("repeated number: ", left[i])
						# there might be a sequence
						first_rep = i
						# last_seq = i - 1
						maybe_seq = left[0:first_rep]
						# print(maybe_seq)
						if len(numstr) % len(maybe_seq) == 0:
							needed_reps = int(len(numstr) / len(maybe_seq))
							# print(f"sequence {maybe_seq} needs to repeat {needed_reps} to equal {numstr}")
							if maybe_seq * needed_reps == numstr:
								print(f"found {maybe_seq} in {numstr}")
								invalid_ids += i_outer
							# else:
							# 	print(f"{maybe_seq} not repeating correctly for {numstr}")
						break

print(invalid_ids)
