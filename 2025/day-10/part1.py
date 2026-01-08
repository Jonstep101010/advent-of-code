import itertools
import sys

if len(sys.argv) > 1:
	INPUT = sys.argv[1]
else:
	INPUT = "input1.txt"


def button_combination_patterns(
	coeffs: list[tuple[int, ...]],
) -> dict[tuple[int, ...], dict[tuple[int, ...], int]]:
	"""
	Group all possible button combinations by parity pattern (for indicators this IS the effect).

	For each parity pattern, store the minimal button press count needed to achieve it.
	"""
	num_indicators = len(coeffs[0])
	patterns_by_parity = {}

	# Try all 2^n subsets of buttons
	for num_buttons in range(len(coeffs) + 1):
		for button_indices in itertools.combinations(range(len(coeffs)), num_buttons):
			# Sum coefficients for selected buttons modulo 2 (XOR for binary)
			effect = tuple(
				sum(coeffs[i][j] for i in button_indices) % 2
				for j in range(num_indicators)
			)

			# Store minimal press count for this effect
			if effect not in patterns_by_parity:
				patterns_by_parity[effect] = num_buttons

	return patterns_by_parity


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

	def process(
		indicator_seq: tuple[bool, ...],
		button_seq: list[tuple[int, ...]],
	) -> int:
		"""
		Find minimal button presses using precomputed combinations.
		For boolean indicators, each button combination produces a unique XOR pattern.
		"""
		# Convert indicator bools to integers (False=0, True=1)
		goal = tuple(int(b) for b in indicator_seq)

		# Convert buttons to binary coefficients (1 if button affects indicator, 0 otherwise)
		coeffs = [
			tuple(int(i in button) for i in range(len(indicator_seq)))
			for button in button_seq
		]

		patterns = button_combination_patterns(coeffs)

		# For boolean toggle problems, we just need to find the pattern that matches our goal
		if goal in patterns:
			return patterns[goal]
		else:
			return float("inf")

	print(
		sum([process(machines[i][0], machines[i][1]) for i, _ in enumerate(machines)])
	)
