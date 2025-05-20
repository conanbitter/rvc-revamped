import math


def split_evenly(arr, N):
    L = len(arr)
    parts = []
    prev = 0
    for i in range(N):
        # compute next boundary with ceil
        curr = math.ceil((i + 1) * L / N)
        parts.append(arr[prev:curr])
        prev = curr
    return parts


def spread_levels(n, levels, length):
    # curr = math.ceil((n + 1) * length / levels)
    return math.floor(n / (length / levels))


# example
# data = list(range(1, 18))  # 1–18
# chunks = split_evenly(data, 4)
# print([len(c) for c in chunks])  # → [5, 4, 5, 4]
# print(chunks)
N = 23
L = 5
data = [spread_levels(i, L, N) for i in range(0, N)]
splist = [0] * L
for lv in data:
    splist[lv] += 1

print(data)
print(splist)
