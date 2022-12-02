#! /usr/bin/env python3

import sys

score = 0

# A/B/C = Rock/Paper/Scissors

# X/Y/Z = win/loss/draw

moves = {'A': 'R',
         'B': 'P',
         'C': 'S'}

playscores = {'R': 1, 'P': 2, 'S': 3}

beats = {'R': 'S', 'S': 'P', 'P': 'R'}

revbeats = {beats[k]: k for k in beats.keys()}

print(beats)
print(revbeats)

for line in sys.stdin:
	(oppo, game) = line.split()

	oppo = moves[oppo]

	# now we need a win, loss or draw
	play = ""
	if game == 'X':
		# need to lose
		# play whatever the oppo move would beat
		play = beats[oppo]
	elif game == 'Y':
		# need to draw
		# play same as oppo
		play = oppo
	else:
		# need to win
		# play whatever beats oppo
		# need to reverse lookup
		play = revbeats[oppo]

	# score is the sum of what you played...
	score += playscores[play]

	# ... and the round outcome
	if oppo == beats[play]:
		score += 6
	elif oppo == play:
		score += 3

print(score)
