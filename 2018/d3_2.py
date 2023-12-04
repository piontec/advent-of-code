import sys


fabric = {(i,j):0 for i in range(1000) for j in range(1000)}

with open("i3.txt") as f:
    lines = f.readlines()

for line in lines:
    s = line.split(' ')
    start = s[2].rstrip(":").split(",")
    size = s[3].split("x")
    startx, starty = int(start[0]), int(start[1])
    sizex, sizey = int(size[0]), int(size[1])
    for x in range(startx, startx+sizex):
        for y in range(starty, starty+sizey):
            fabric[(x,y)]+=1

for line in lines:
    s = line.split(' ')
    start = s[2].rstrip(":").split(",")
    size = s[3].split("x")
    startx, starty = int(start[0]), int(start[1])
    sizex, sizey = int(size[0]), int(size[1])
    only1 = True
    for x in range(startx, startx+sizex):
        for y in range(starty, starty+sizey):
            if fabric[(x,y)] != 1:
                only1 = False
                break
        if not only1:
            break
    if only1:
        print(s[0])
        sys.exit(0)

