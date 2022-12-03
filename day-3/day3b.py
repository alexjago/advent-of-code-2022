#! /usr/bin/env python3

import sys

prioritysum = 0

# now we need 3 groups of rucksacks
counter = 0

groups = []

for line in sys.stdin:
    groups.append(set(line.strip()))


for k in range(len(groups)//3):
    a = groups[3*k]
    b = groups[3*k + 1]
    c = groups[3*k + 2]

    item = a.intersection(b).intersection(c).pop()

    # Lowercase item types a through z have priorities 1 through 26.
    # Uppercase item types A through Z have priorities 27 through 52.

    if item.islower():
        prioritysum += 1 + ord(item) - ord('a')
    else:
        prioritysum += 27 + ord(item) - ord('A')

print(prioritysum)

