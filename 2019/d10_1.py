from typing import Dict, List, Tuple
import sys

Point = Tuple[int, int]


def parse(lines: List[str]) -> List[Point]:
    res: List[Point] = []
    for y in range(len(lines)):
        for x in range(len(lines[y])):
            if lines[y][x] == "#":
                res.append((x, y))
    return res


def visible(ast: Point, asteroids: List[Point]) -> int:
    lines: Dict[Point, List[Point]] = {}
    for a in asteroids:
        if a == ast:
            continue
        if a[0] == ast[0]:
            co_a = co_b = sys.maxsize  # fake value to mark vertical line
        else:
            co_a = (a[1] - ast[1]) / (a[0] - ast[0])
            co_b = a[1] - ((a[1] - ast[1])/(a[0] - ast[0]) * a[0])
#            assert ast[1] == co_a * ast[0] + co_b
#            assert a[1] == co_a * a[0] + co_b
        line = (co_a, co_b)
        if line in lines.keys():
            lines[line].append(a)
        else:
            lines[line] = [a]
    return len(lines)


def run(lines: List[str]) -> Tuple[Tuple[int, int], int]:
    asteroids = parse(lines)
    res = {ast: visible(ast, asteroids) for ast in asteroids}
    best = max(res, key=res.get)
    return best, res[best]


lines = """.#..#
.....
#####
....#
...##""".split('\n')
res = run(lines)

with open("i10.txt") as f:
    lines = f.readlines()
res = run(lines)
print("Part 1: {}".format(res))
