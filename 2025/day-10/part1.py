import sys

if len(sys.argv) > 1:
	INPUT = sys.argv[1]
else:
	INPUT = "input1.txt"

with open(INPUT) as f:
	machines_input = [line.strip().split(" ") for line in f.readlines()]
	machines = []
	for m in machines_input:
		indicator_seq = []
		for c in m[0][1:-1]:
			match c:
				case "#":
					indicator_seq.append(True)
				case ".":
					indicator_seq.append(False)
				case _:
					assert False
		seq_buttons = [
			tuple(int(j) for j in mj.strip("()").split(",")) for mj in m[1:-1]
		]
		seq_joltages = [int(j) for j in m[-1].strip("{}").split(",")]
		machines.append((tuple(indicator_seq), seq_buttons, seq_joltages))
	# indicators are initially off

	def process(
		indicator_seq: tuple[bool, ...],
		button_seq: list[tuple[int, ...]],
	) -> int:
		machine_total = 0
		machine_set = set()
		state_off = tuple(False for _ in range(len(indicator_seq)))
		machine_set.add(state_off)
		while True:
			new_set = set()
			for state in machine_set:
				for button in button_seq:
					state_list = list(state)
					for bit in button:
						state_list[bit] = not state_list[bit]
					new_set.add(tuple(state_list))
			machine_set = new_set
			machine_total += 1
			if indicator_seq in machine_set:
				return machine_total

	print(
		sum(
			[
				process(machines[i][0], machines[i][1])
				for i, _ in enumerate(machines_input)
			]
		)
	)
