import sys

default_input = "input1.txt"

if len(sys.argv) > 1:
	INPUT = sys.argv[1]
else:
	INPUT = "input1.txt"

password = 0
dial_position = 50

with open(INPUT) as input:
	output = open(INPUT.replace("input", "output"), "w")
	for action in input.readlines():
		if action.startswith("R"):
			action = int(action[1:])
			full, partial = divmod(abs(action), 100)
			new = dial_position + partial
		elif action.startswith("L"):
			action = -int(action[1:])
			full, partial = divmod(abs(action), 100)
			new = dial_position - partial
		passed = int(dial_position != 0 and not (0 < new < 100))
		dial_position = new % 100
		password += full + passed
		output.write(str(dial_position) + "\n")

output.close()
print(password)
