import pprint
from collections import deque


def closest(grid, coords, start, min_grid, max_grid):
    visited = set()
    found = []
    found_dist = -1
    to_visit = deque([(start, 0)])
    while True:
        if len(to_visit) == 0:
            break
        p, dist = to_visit.popleft() 
        visited.add(p)    
        if found_dist > -1 and dist > found_dist:
            continue            
        if p in coords:
            found_dist = dist
            f = (coords.index(p), dist)
            if f not in found:
                found.append(f)
            continue
        next_points = [(n, dist+1) for n in get_neighbors(p, visited, min_grid, max_grid)]
        for np in next_points:
            if np not in to_visit:
                to_visit.append(np)
    return found


def get_neighbors(p, visited, min_grid, max_grid):
    left = (p[0]-1, p[1])
    right = (p[0]+1, p[1])
    up = (p[0], p[1]-1)
    down = (p[0], p[1]+1)
    result = []
    if left not in visited and left[0] >= min_grid[0]:
        result.append(left)
    if right not in visited and right[0] <= max_grid[0]:
        result.append(right)
    if up not in visited and up[1] >= min_grid[1]:
        result.append(up)
    if down not in visited and down[1] <= max_grid[1]:
        result.append(down)
    return result


with open("i6.txt") as f:
    lines = f.readlines()

pp = pprint.PrettyPrinter(indent=4)
# test case
# lines = ["1, 1", "1, 6", "8, 3", "3, 4", "5, 5", "8, 9"]

coords = [(int(line.split(",")[0].strip()), int(line.split(",")[1].strip())) for line in lines]
min_x = min([x for (x,y) in coords])
min_y = min([y for (x,y) in coords])
max_x = max([x for (x,y) in coords])
max_y = max([y for (x,y) in coords])
grid = {(x,y):-1 for y in range(min_y, max_y+1) for x in range(min_x, max_x+1)}

for p in grid:
    print("running closest for {}".format(p))
    grid[p] = closest(grid, coords, p, (min_x, min_y), (max_x, max_y))
    print("result: {}".format(grid[p]))

print("closest done")
points_on_edge = [(x, y) for x in range(min_x, max_x+1) for y in [min_y, max_y]]
points_on_edge.extend([(x, y) for y in range(min_y, max_y+1) for x in [min_x, max_x]])
indexes_on_edge = set([grid[p][0][0] for p in points_on_edge if len(grid[p])==1])

sizes = {i:0 for i in range(len(coords))}
for i in range(len(coords)):
    sizes[i] = sum(1 for points in grid.values() if len(points)==1 and points[0][0]==i) \
        if i not in indexes_on_edge else -1
pp.pprint(sizes)
print("* {}".format(max(sizes.values())))
