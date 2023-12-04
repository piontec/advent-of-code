import sys

with open("i1.txt") as f:
    lines = f.readlines()
nums = [int(x) for x in lines]
nums.sort()

for i1 in range(len(nums)):
    for i2 in range(len(nums)):
        if i1 == i2:
            continue
        if nums[i1] + nums[i2] == 2020:
            print(nums[i1] * nums[i2])
            sys.exit(0)
        if nums[i1] + nums[i2] > 2020:
            break
