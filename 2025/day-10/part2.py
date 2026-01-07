import sys

if len(sys.argv) > 1:
	INPUT = sys.argv[1]
else:
	INPUT = "input2.txt"

with open(INPUT) as f:
	machines_input = [line.strip().split(" ") for line in f.readlines()]
	machines = []
	for m in machines_input:
		# indicator_seq = []
		# for c in m[0][1:-1]:
		# 	match c:
		# 		case "#":
		# 			indicator_seq.append(True)
		# 		case ".":
		# 			indicator_seq.append(False)
		# 		case _:
		# 			assert False
		seq_buttons = [
			tuple(int(j) for j in mj.strip("()").split(",")) for mj in m[1:-1]
		]
		seq_joltages = tuple([int(j) for j in m[-1].strip("{}").split(",")])
		machines.append(((), seq_buttons, seq_joltages))
	# indicators are initially off

	from process2 import process

	print(
		sum([process(machines[i][1], machines[i][2]) for i, _ in enumerate(machines)])
	)
