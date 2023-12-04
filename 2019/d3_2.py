from typing import List, Dict, Tuple, Callable


Point = Tuple[int, int]
WireMap = Dict[Point, List[Point]]


def applyOp(op: str, p: Point) -> Point:
    if op == "U":
        return (p[0], p[1] + 1)
    if op == "D":
        return (p[0], p[1] - 1)
    if op == "L":
        return (p[0] - 1, p[1])
    if op == "R":
        return (p[0] + 1, p[1])


def run(lines: List[str]) -> int:
    wire_id = 0
    area: WireMap = {}
    crossings: WireMap = {}
    for wire in lines:
        wire_id += 1
        length = 0
        pos: Point = (0, 0)
        for step in wire.split(","):
            instr = step[0]
            dist = int(step[1:])
            for s in range(dist):
                new_pos = applyOp(instr, pos)
                length += 1
                if new_pos in area:
                    area[new_pos].append((wire_id, length))
                    crossings[new_pos] = area[new_pos].copy()
                else:
                    area[new_pos] = [(wire_id, length)]
                pos = new_pos
    crs = {k: crossings[k]
           for k in crossings if crossings[k][0][0] != crossings[k][1][0]}
    best = None
    for k in crs:
        dist = crs[k][0][1] + crs[k][1][1]
        if best == None:
            best = dist
        elif dist < best:
            best = dist
    return best


lines = ["R8,U5,L5,D3", "U7,R6,D4,L4"]
assert run(lines) == 30
with open("i3.txt") as f:
    lines = f.readlines()
print(str.format("{}", run(lines)))
