import sys
import functools

if len(sys.argv) > 1:
	INPUT = sys.argv[1]
else:
	INPUT = "input2.txt"


def apply_op(dict_acc_pos: dict[int, int], tuple_y_line):
	# {7: 1} (0, '.......S.......\n')
	new_pos = {}
	_, line = tuple_y_line
	for x, count in dict_acc_pos.items():
		if line[x] == "^":
			new_pos[x - 1] = new_pos.get(x - 1, 0) + count
			new_pos[x + 1] = new_pos.get(x + 1, 0) + count
		else:
			new_pos[x] = new_pos.get(x, 0) + count
	return new_pos


with open(INPUT) as f:
	filestr = f.readlines()
	new_beam_insertions = None
	for y, line in enumerate(filestr):
		if "S" in line:
			new_beam_insertions = {line.index("S"): 1}
			break
	print(
		sum(
			functools.reduce(apply_op, enumerate(filestr), new_beam_insertions).values()
		)
	)
