def fuel_weight(x):
    res = 0
    current = x
    while True:
        needed = current / 3 - 2
        if needed <= 0:
            break
        res += needed
        current = needed
    return res

with open("i1.txt") as f:
    lines = f.readlines()
nums = [fuel_weight(int(x)) for x in lines]
print(sum(nums))
