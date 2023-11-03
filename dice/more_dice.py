from itertools import product

DICE = [
    (2, 6),
    (1, 8),
    (2, 20),
]

BY_PROBABILITY = True

dice =  ()
lowest = 0
highest = 0
for die in DICE:
    dice += tuple(tuple(range(1, die[1] + 1)) for _ in range(die[0]))
    lowest += die[0]
    highest += die[1] * die[0]

pools = []
totals = {key: 0 for key in range(lowest, highest + 1)}
for roll in product(*dice):
    pools += (roll,)
    totals[sum(roll)] += 1
total = len(pools)

totals = {key: (value/len(pools), value) for key, value in totals.items()}

if BY_PROBABILITY:
    sort = lambda item: item[1][0]
else:
    sort = lambda item: item[0]

totals = {key: value for key, value in sorted(totals.items(), key=sort)}

for key, v in totals.items():
    print(f"{key:04d}: {v[0]*100:02.2f} ({v[1]}/{total})")
print(f"Rolling {" and ".join([f'{die[0]}d{die[1]}' for die in DICE])}")
