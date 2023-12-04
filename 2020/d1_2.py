import sys

with open("i1.txt") as f:
    lines = f.readlines()
nums = [int(x) for x in lines]
nums.sort()

for i1 in range(len(nums)):
    for i2 in range(len(nums)):
        for i3 in range(len(nums)):
            if i1 == i2 or i2 == i3 or i1 == i3:
                continue
            if nums[i1] + nums[i2] + nums[i3] == 2020:
                print(nums[i1] * nums[i2] * nums[i3])
                sys.exit(0)
            if nums[i1] + nums[i2] + nums[i3] > 2020:
                break
