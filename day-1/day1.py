#! /usr/bin/env python3

import sys

# we have an input file of newline-delimited numbers
# empty line = new Elf

maximum = 0
current = 0

all_elves = []

for line in sys.stdin:
	try:
		current += int(line)
	except Exception:
		all_elves.append(current)
		current = 0


all_elves.sort()

print("Part 1:\t", all_elves[-1])
print("Part 2:\t", sum(all_elves[-3:]))

