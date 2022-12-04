#! /usr/bin/env python3

import sys

total = 0

groups = []

for line in sys.stdin:
	(l,r) = line.strip().split(',')
	(aa,bb) = l.split('-')
	(cc,dd) = r.split('-')

	a = int(aa)
	b = int(bb)
	c = int(cc)
	d = int(dd)

	if (a <= c and b >= d) or (c <= a and d >= b):
		total += 1

print(total)


