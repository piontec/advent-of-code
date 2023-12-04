import copy
from typing import List, Tuple, Dict

import numpy


class Area:
    def __init__(self, lines: List[str]):
        area: List[List[str]] = []
        for line in lines:
            area.append([c for c in line.strip()])
        self._array = numpy.array(area)

    def print(self) -> None:
        for row in self._array:
            for pos in row:
                print(pos)
            print("\n")

    @property
    def array(self) -> numpy.array:
        return self._array

    def get_copy(self) -> numpy.array:
        return copy.deepcopy(self._array)

    def run_step(self) -> bool:
        copy = numpy.copy(self._array)
        changed = False
        dy, dx = self._array.shape
        for y in range(dy):
            for x in range(dx):
                if copy[y, x] == ".":
                    continue
                if copy[y, x] == "L" and self._no_occupied_around(x, y, copy):
                    self._array[y, x] = "#"
                    changed = True
                if copy[y, x] == "#" and self._at_least_busy(x, y, copy, 5):
                    self._array[y, x] = "L"
                    changed = True
        return changed

    def _at_least_busy(self, x: int, y: int, arr: numpy.array, min_val: int) -> bool:
        res = self._whats_in_all_directions(x, y, arr)
        return res["#"] >= min_val

    def _no_occupied_around(self, x: int, y: int, arr: numpy.array) -> bool:
        res = self._whats_in_all_directions(x, y, arr)
        return res["#"] == 0

    def _whats_in_all_directions(self, x: int, y: int, arr: numpy.array) -> Dict[str, int]:
        dir8: List[str] = [self._whats_in_direction(x, y, arr, d) for d in
                           [(-1, 0), (-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1)]]
        all_dirs = {s: sum([d == s for d in dir8]) for s in [".", "L", "#"]}
        return all_dirs

    def _whats_in_direction(self, x: int, y: int, arr: numpy.array, direction: Tuple[int, int]) -> str:
        dy, dx = arr.shape
        cy = y
        cx = x
        while True:
            cy += direction[0]
            cx += direction[1]
            if cx < 0 or cx >= dx or cy < 0 or cy >= dy:
                return "."
            if arr[cy, cx] != ".":
                return arr[cy, cx]


def run_sim(area: Area) -> int:
    sim_step = 0
    while area.run_step():
        sim_step += 1
        print(f"Completed step {sim_step}")
    res = sum([sum([pos == "#" for pos in row]) for row in area.array])
    return res


see_test_1 = """.............
.L.L.#.#.#.#.
.............
"""
a = Area(see_test_1.splitlines())
ra = a._whats_in_all_directions(1, 1, a._array)
assert ra == {"L": 1, "#": 0, ".": 7}

see_test_2 = """.......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....
"""
a = Area(see_test_2.splitlines())
ra = a._whats_in_all_directions(3, 4, a._array)
assert ra == {"L": 0, "#": 8, ".": 0}

test1 = """L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"""

res = run_sim(Area(test1.splitlines()))
assert res == 26

with open("i11.txt", "r") as f:
    lines = f.readlines()
res = run_sim(Area(lines))
print(res)
