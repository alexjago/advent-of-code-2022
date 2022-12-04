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

	# for part B we just need any overlap
	# c in [a,b] OR b in [c,d] ... was too low
	# ... OR d in [a,b] OR b in [c,d]
	if (a <= c and c <= b) or (c <= b and b <= d) or \
		(a <= d and d <= b) or (c <= a and a <= d) :
		total += 1

print(total)


