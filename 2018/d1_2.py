with open("i1.txt") as f:
    lines = f.readlines()
nums = [int(x) for x in lines]

s = set()
res = 0
found = False
while True:
    for num in nums:
       res += num 
       if res in s:
           print("* "+str(res))
           found = True
           break           
       s.add(res)
    if found:
        break
