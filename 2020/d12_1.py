from typing import List, Tuple

Heading = Tuple[int, int]

headings = [(1, 0), (0, -1), (-1, 0), (0, 1)]


def rotate(heading: Heading, inst: str, arg: int) -> Heading:
    current_heading = headings.index(heading)
    cnt = arg // 90
    cw = 1 if inst == "R" else -1
    new_heading = headings[(current_heading + cw * cnt) % len(headings)]
    return new_heading


def run_sim(lines: List[str]) -> int:
    pos: List[int, int] = [0, 0]
    heading: Heading = (1, 0)
    for line in lines:
        inst, arg = line[0], int(line[1:])
        if inst == "N":
            pos[1] += arg
        elif inst == "S":
            pos[1] -= arg
        elif inst == "E":
            pos[0] += arg
        elif inst == "W":
            pos[0] -= arg
        elif inst == "F":
            pos[0] += heading[0] * arg
            pos[1] += heading[1] * arg
        elif inst in ["L", "R"]:
            heading = rotate(heading, inst, arg)
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
assert res == 25

with open("i12.txt", "r") as f:
    lines = f.readlines()
res = run_sim(lines)
print(res)
