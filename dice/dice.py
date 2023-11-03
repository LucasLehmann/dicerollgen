"""Generate statistics for dice rolls."""
from itertools import product
DICE_COUNT = 3
DICE_SIDES = 6

SHOW_STATS = True
STAT_SORT = "probability"
SORTING_METHODS = ("probability", "count", "key", None)

COMBINATIONS = DICE_SIDES**DICE_COUNT

sum_counts = {i: 0 for i in range(DICE_COUNT, DICE_COUNT*DICE_SIDES + 1)}
for roll in product(range(1, DICE_SIDES + 1), repeat=DICE_COUNT):
    sum_counts[sum(roll)] += 1

stats = {key: (value / COMBINATIONS, sum_counts[key]) for key, value in sum_counts.items()}
sum_ = max(stats, key=stats.get)
dice_ = f"{DICE_COUNT}d{DICE_SIDES}"
sum_info_ = f"{sum_} at {stats[sum_][0]:0.2%} or {stats[sum_][1]:02d}/{COMBINATIONS}"
print(f"Most likely sum of {dice_} is {sum_info_} times.")

if SHOW_STATS and STAT_SORT:
    if STAT_SORT == "probability":
        sort = lambda item: item[1][0]
    elif STAT_SORT == "count":
        sort = lambda item: item[1][1]
    elif STAT_SORT == "key":
        sort = lambda item: item[0]
    else:
        sort = lambda item: item[1]
        print(f"{STAT_SORT} is not a valid sort method. Sorting by probability.")

    stats = {key: value for key, value in sorted(stats.items(), key=sort, reverse=True)}
    method_ = STAT_SORT if STAT_SORT in SORTING_METHODS else "probability"
    odds_ = [f"{key:03d}: {value[0]:0.2%}({value[1]:02d}/{COMBINATIONS})" for key, value in stats.items()]
    print(f"Sums by {method_}: \n" + "\n".join(odds_))
