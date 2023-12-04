import sys


with open("i2.txt") as f:
    lines = f.readlines()

twos = 0
threes = 0
for line in lines:
    letters = list(line)
    f = {}
    for l in letters:
        if l in f.keys():
            f[l]+=1
        else:
            f[l]=1
    has_2, has_3 = False, False
    for l in f.keys():
        if f[l] == 2:
            has_2 = True
        if f[l] == 3:
            has_3 = True
    if has_2:
        twos += 1
    if has_3:
        threes += 1
print(twos*threes)
