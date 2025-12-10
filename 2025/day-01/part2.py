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
	for line in input.readlines():
		full, partial = divmod(int(line[1:]), 100)
		new = dial_position + partial if line[0] == "R" else dial_position - partial
		password += full + int(dial_position != 0 and not (0 < new < 100))
		dial_position = new % 100
		# output.write(str(dial_position) + "\n")

# output.close()
print(password)
