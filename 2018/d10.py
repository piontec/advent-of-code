from typing import List, Tuple
import sys


with open("i10.txt") as f:
    lines: List[str] = f.readlines()

# lines = """position=< 9,  1> velocity=< 0,  2>
# position=< 7,  0> velocity=<-1,  0>
# position=< 3, -2> velocity=<-1,  1>
# position=< 6, 10> velocity=<-2, -1>
# position=< 2, -4> velocity=< 2,  2>
# position=<-6, 10> velocity=< 2, -2>
# position=< 1,  8> velocity=< 1, -1>
# position=< 1,  7> velocity=< 1,  0>
# position=<-3, 11> velocity=< 1, -2>
# position=< 7,  6> velocity=<-1, -1>
# position=<-2,  3> velocity=< 1,  0>
# position=<-4,  3> velocity=< 2,  0>
# position=<10, -3> velocity=<-1,  1>
# position=< 5, 11> velocity=< 1, -2>
# position=< 4,  7> velocity=< 0, -1>
# position=< 8, -2> velocity=< 0,  1>
# position=<15,  0> velocity=<-2,  0>
# position=< 1,  6> velocity=< 1,  0>
# position=< 8,  9> velocity=< 0, -1>
# position=< 3,  3> velocity=<-1,  1>
# position=< 0,  5> velocity=< 0, -1>
# position=<-2,  2> velocity=< 2,  0>
# position=< 5, -2> velocity=< 1,  2>
# position=< 1,  4> velocity=< 2,  1>
# position=<-2,  7> velocity=< 2, -2>
# position=< 3,  6> velocity=<-1, -1>
# position=< 5,  0> velocity=< 1,  0>
# position=<-6,  0> velocity=< 2,  0>
# position=< 5,  9> velocity=< 1, -2>
# position=<14,  7> velocity=<-2,  0>
# position=<-3,  6> velocity=< 2, -1>""".splitlines()

Pair = Tuple[int, int]

class Point:
    def __init__(self, pos: Pair, vel: Pair):
        self.pos: Pair = pos
        self.vel: Pair = vel

    def move(self):
        new_pos = (self.pos[0] + self.vel[0], self.pos[1] + self.vel[1])
        return Point(new_pos, self.vel)

    def move_back(self):
        new_pos = (self.pos[0] - self.vel[0], self.pos[1] - self.vel[1])
        return Point(new_pos, self.vel)
    

def print_points(points: List[Point]):
    min_x = min(p.pos[0] for p in points)
    max_x = max(p.pos[0] for p in points)
    min_y = min(p.pos[1] for p in points)
    max_y = max(p.pos[1] for p in points)
    print(max_y - min_y)    
    for y in range(min_y, max_y + 1):
        for x in range(min_x, max_x + 1):
            if (x, y) in [p.pos for p in points]:
                print("#", end='')
            else:
                print(".", end='')
        print("")


points: List[Point] = []
for line in lines:
    t1 = line.split(">")    
    t2_1 = t1[0].split("<")
    nums = t2_1[1].split(",")
    pos = (int(nums[0].strip()), int(nums[1].strip()))
    t2_2 = t1[1].split("<")
    nums = t2_2[1].split(",")
    vel = (int(nums[0].strip()), int(nums[1].strip()))
    points.append(Point(pos, vel))

second = 0
height = sys.maxsize
while True:
    second += 1
    points = [p.move() for p in points]

    min_x, max_x = min(p.pos[0] for p in points), max(p.pos[0] for p in points)
    min_y, max_y = min(p.pos[1] for p in points), max(p.pos[1] for p in points)    
    new_height = max_y - min_y
    print(new_height)
    if new_height < height:
        height = new_height
    else:        
        found = [p.move_back() for p in points]
        print_points(found)
        print(second-1)
        break

