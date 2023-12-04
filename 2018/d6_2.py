from typing import Tuple, List
import pprint


Point = Tuple[int, int]

def dist(p1: Point, p2: Point) -> int:
    return abs(p1[0] - p2[0]) + abs(p1[1] - p2[1])    

def sum_dist(p: Point, coords: List[Point]) -> int:
    return sum(dist(p, c) for c in coords)

with open("i6.txt") as f:
    lines = f.readlines()

# test case
# lines = ["1, 1", "1, 6", "8, 3", "3, 4", "5, 5", "8, 9"]

pp = pprint.PrettyPrinter(indent=4)
thr = 10000

coords: List[Point] = [(int(line.split(",")[0].strip()), int(line.split(",")[1].strip())) for line in lines]
min_x = min([x for (x,y) in coords])
min_y = min([y for (x,y) in coords])
max_x = max([x for (x,y) in coords])
max_y = max([y for (x,y) in coords])
grid = {(x,y):sum_dist((x, y), coords) for y in range(min_y, max_y+1) for x in range(min_x, max_x+1)}
close = {p:1 if d < thr else 0 for p, d in grid.items()}
print(sum(close.values()))
