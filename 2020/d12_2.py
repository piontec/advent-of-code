from typing import List

import numpy


def rotate(waypoint: numpy.array, inst: str, arg: int) -> numpy.array:
    cnt = arg // 90
    cw = numpy.array([1, -1]) if inst == "R" else numpy.array([-1, 1])
    for i in range(cnt):
        waypoint[0], waypoint[1] = waypoint[1], waypoint[0]
        waypoint *= cw
    return waypoint


def run_sim(lines: List[str]) -> int:
    pos = numpy.array([0, 0])
    waypoint = numpy.array([10, 1])
    for line in lines:
        inst, arg = line[0], int(line[1:])
        if inst == "N":
            waypoint[1] += arg
        elif inst == "S":
            waypoint[1] -= arg
        elif inst == "E":
            waypoint[0] += arg
        elif inst == "W":
            waypoint[0] -= arg
        elif inst == "F":
            pos += arg * waypoint
        elif inst in ["L", "R"]:
            waypoint = rotate(waypoint, inst, arg)
        else:
            raise Exception("Unknown instruction")
    return abs(pos[0]) + abs(pos[1])


test1 = """F10
N3
F7
R90
F11
"""
res = run_sim(test1.splitlines())
assert res == 286

with open("i12.txt", "r") as f:
    lines = f.readlines()
res = run_sim(lines)
print(res)
