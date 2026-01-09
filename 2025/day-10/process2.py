from functools import cache
import itertools


def button_combination_patterns(
	coeffs: list[tuple[bool, ...]],
) -> dict[tuple[bool, ...], dict[tuple[int, ...], int]]:
	"""
	Group all possible button combinations by parity pattern.

	For each parity pattern, store the minimal button press count needed to achieve it.
	"""
	num_joltages = len(coeffs[0])
	patterns_by_parity = {
		parity: {} for parity in itertools.product(range(2), repeat=num_joltages)
	}

	# Try all 2^n subsets of buttons
	for num_buttons in range(len(coeffs) + 1):
		for button_indices in itertools.combinations(range(len(coeffs)), num_buttons):
			# Sum coefficients for selected buttons
			effect = tuple(
				sum(coeffs[i][j] for i in button_indices) for j in range(num_joltages)
			)
			parity = tuple(bool(e % 2) for e in effect)

			# Store minimal press count for this effect
			if effect not in patterns_by_parity[parity]:
				patterns_by_parity[parity][effect] = num_buttons

	return patterns_by_parity


def process(button_seq: list[tuple[int, ...]], joltage_seq: tuple[int, ...]) -> int:
	"""
	Solve using recursive halving: phase1 (odd presses), phase2 (even presses halved).

	algorithm adapted from https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/
	"""
	# Convert buttons to binary coefficients (1 if button affects light, 0 otherwise)
	coeffs = [
		tuple(bool(i in button) for i in range(len(joltage_seq)))
		for button in button_seq
	]

	patterns_by_parity = button_combination_patterns(coeffs)

	@cache
	def solve(goal: tuple[int, ...]) -> int:
		# Base case: all joltages are 0
		if all(g == 0 for g in goal):
			return 0
		min_presses = float("inf")
		# Get patterns matching goal's parity
		parity = tuple(bool(g % 2) for g in goal)
		phase1_candidates = patterns_by_parity[parity].items()
		for phase1_effect, phase1_presses in phase1_candidates:
			# Check if phase1 effect is achievable (all components <= goal)
			if all(e <= g for e, g in zip(phase1_effect, goal)):
				# Phase 2: recursively solve with halved remaining goal (doubled for both directions)
				phase2_goal = tuple((g - e) // 2 for e, g in zip(phase1_effect, goal))
				phase2_presses = 2 * solve(phase2_goal)
				min_presses = min(min_presses, phase1_presses + phase2_presses)

		return min_presses

	return solve(joltage_seq)
