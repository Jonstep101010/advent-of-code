import sys

default_input = "input1.txt"
actions = []

if len(sys.argv) > 1:
	INPUT = sys.argv[1]
else:
	INPUT = "input1.txt"

with open(INPUT) as input:
	output = open(INPUT.replace("input", "output"), "w")
	for action in input.readlines():
		if action.startswith("R"):
			action_num = int(action[1:])
			# print(f"add {action_num}")
		elif action.startswith("L"):
			action_num = -int(action[1:])
			# print(f"subtract {action_num}")
		actions.append(action_num)

# number of times dial points at 0, starting from 50
# 0 <= 50 <= 99
password = 0
dial_position = 50

for action in actions:
	dial_position = (dial_position + action) % 100
	if dial_position == 0:
		password += 1
	output.write(str(dial_position) + "\n")

output.close()
print(password)
