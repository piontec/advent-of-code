import copy
from typing import List

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
                if copy[y, x] == "#" and self._at_least_busy(x, y, copy, 4):
                    self._array[y, x] = "L"
                    changed = True
        return changed

    def _at_least_busy(self, x: int, y: int, arr: numpy.array, min_val: int) -> bool:
        dy, dx = arr.shape
        busy_cnt = 0
        for cx in range(x - 1, x + 2):
            for cy in range(y - 1, y + 2):
                if cx == x and cy == y:
                    continue
                if cx < 0 or cx >= dx or cy < 0 or cy >= dy:
                    continue
                if arr[cy, cx] == "#":
                    busy_cnt += 1
                    if busy_cnt >= min_val:
                        return True
        return False

    def _no_occupied_around(self, x: int, y: int, arr: numpy.array) -> bool:
        dy, dx = arr.shape
        for cx in range(x - 1, x + 2):
            for cy in range(y - 1, y + 2):
                if cx == x and cy == y:
                    continue
                if cx < 0 or cx >= dx or cy < 0 or cy >= dy:
                    continue
                if arr[cy, cx] == "#":
                    return False
        return True


def run_sim(area: Area) -> int:
    sim_step = 0
    while area.run_step():
        sim_step += 1
        print(f"Completed step {sim_step}")
    res = sum([sum([pos == "#" for pos in row]) for row in area.array])
    return res


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
assert res == 37

with open("i11.txt", "r") as f:
    lines = f.readlines()
res = run_sim(Area(lines))
print(res)
