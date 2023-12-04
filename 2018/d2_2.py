import sys


with open("i2.txt") as f:
    lines = f.readlines()

for ln1 in range(0, len(lines)):
    l1 = lines[ln1]
    for ln2 in range(ln1+1, len(lines)):
        l2 = lines[ln2]
        lc1 = list(l1)
        lc2 = list(l2)
        if len(lc1) != len(lc2):
            print("different lengths")
            sys.exit(1)
        diff1 = -1
        diff2 = -1
        for cc in range(0, len(lc1)):
            if lc1[cc] != lc2[cc]:
                if diff1 == -1:
                    diff1 = cc
                    continue
                diff2 = cc
                break
        if diff1 != -1 and diff2 == -1:
            # print(l1)
            # print(l2)
            print("{}{}".format(''.join(lc1[:diff1]), ''.join(lc1[diff1+1:])))
            sys.exit(0)

