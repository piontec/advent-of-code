with open("i1.txt") as f:
    lines = f.readlines()
nums = [int(x) for x in lines]
print(sum(nums))
