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
		seq_buttons = [
			tuple(int(j) for j in mj.strip("()").split(",")) for mj in m[1:-1]
		]
		wiring_schematics.append(seq_buttons)
		seq_joltages = [int(j) for j in m[-1].strip("{}").split(",")]
		joltages.append(seq_joltages)
		machines.append((indicator_seqs[-1], wiring_schematics[-1], joltages[-1]))
	# indicators are initially off

	def process(machine) -> int:
		machine_total = 0
		state_org = tuple(False for _ in range(len(machine[0])))
		tuple_state = tuple(machine[0])
		machine_set = set()
		machine_set.add(state_org)
		while True:
			new_set = set()
			for state in machine_set:
				for button in machine[1]:
					state_list = list(state)
					for bit in button:
						state_list[bit] = not state_list[bit]
					new_set.add(tuple(state_list))
			machine_set = new_set
			machine_total += 1
			if tuple_state in machine_set:
				return machine_total

	print(sum([process(machines[i]) for i, _ in enumerate(machines_input)]))
