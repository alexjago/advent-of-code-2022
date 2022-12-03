#! /usr/bin/env python3

import sys

prioritysum = 0

for line in sys.stdin:
	# split line in half, do set difference
	l = line.strip()
	n = len(l) >> 1 # halfway through
	left = set(l[:n])
	right = set(l[n:])

	item = left.intersection(right).pop()

	# Lowercase item types a through z have priorities 1 through 26.
	# Uppercase item types A through Z have priorities 27 through 52.

	if item.islower():
		prioritysum += 1 + ord(item) - ord('a')
	else:
		prioritysum += 27 + ord(item) - ord('A')

print(prioritysum)
