#! /usr/bin/env python3

import sys

score = 0

# A/B/C and X/Y/Z = Rock/Paper/Scissors

moves = {'A': 'R', 'X': 'R',
         'B': 'P', 'Y': 'P',
         'C': 'S', 'Z': 'S'}

playscores = {'R': 1, 'P': 2, 'S': 3}

beats = {'R': 'S', 'S': 'P', 'P': 'R'}

for line in sys.stdin:
	(oppo, play) = line.split()

	oppo = moves[oppo]
	play = moves[play]

	# score is the sum of what you played...
	score += playscores[play]

	# ... and the round outcome
	if oppo == beats[play]:
		score += 6
	elif oppo == play:
		score += 3

print(score)
