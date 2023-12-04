with open("i1.txt") as f:
    lines = f.readlines()
nums = [int(x)/3-2 for x in lines]
print(sum(nums))
