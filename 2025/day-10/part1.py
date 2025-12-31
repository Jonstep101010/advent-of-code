import sys

if len(sys.argv) > 1:
	INPUT = sys.argv[1]
else:
	INPUT = "input1.txt"

with open(INPUT) as f:
	# filestr = f.read()
	machines_input = [line.strip().split(" ") for line in f.readlines()]
	indicator_seqs = []  # indicator_diagrams
	wiring_schematics = []
	joltages = []
	machines = []
	for m in machines_input:
		seq_str = m[0][1:-1]
		seq = []
		for c in seq_str:
			match c:
				case "#":
					seq.append(True)
				case ".":
					seq.append(False)
				case _:
					assert False
		indicator_seqs.append(seq)
		seq_tuples_str = m[1:-1]
		seq_tuples = [
			tuple(int(j) for j in mj.strip("()").split(",")) for mj in seq_tuples_str
		]
		wiring_schematics.append(seq_tuples)
		seq_joltages = [int(j) for j in m[-1].strip("{}").split(",")]
		joltages.append(seq_joltages)
		machines.append((indicator_seqs[-1], wiring_schematics[-1], joltages[-1]))
	# indicators are initially off
	for i, _ in enumerate(machines_input):
		print(machines[i])
