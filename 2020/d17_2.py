from typing import List

import numpy


def count_active_neighbors(space: numpy.array, w: int, z: int, y: int, x: int) -> int:
    active = 0
    for nw in range(w - 1, w + 2):
        for nz in range(z - 1, z + 2):
            for ny in range(y - 1, y + 2):
                for nx in range(x - 1, x + 2):
                    if nw == w and nz == z and ny == y and nx == x:
                        continue
                    try:
                        if space[nw, nz, ny, nx] == 1:
                            active += 1
                    except IndexError:
                        pass
    return active


class Space:
    def __init__(self, lines: List[str]):
        area: List[List[int]] = []
        for line in lines:
            area.append([1 if c == "#" else 0 for c in line.strip()])
        self._array = numpy.array([[area]])

    def print(self) -> None:
        print(self._array)

    @property
    def array(self) -> numpy.array:
        return self._array

    def expand_space(self) -> numpy.array:
        array = self._array
        dw, dz, dy, dx = array.shape
        x_min_cube = array[:, : , :, 0]
        x_max_cube = array[:, :, :, dx - 1]
        y_min_cube = array[:, :, 0, :]
        y_max_cube = array[:, :, dy - 1, :]
        z_min_cube = array[:, 0, :, :]
        z_max_cube = array[:, dz - 1, :, :]
        w_min_cube = array[0, :, :, :]
        w_max_cube = array[dw - 1, :, :, :]
        if any(val == 1 for plane in w_min_cube for row in plane for val in row):
            array = numpy.concatenate((numpy.zeros((1, dz, dy, dx), dtype=int), array), axis=0)
            dw, dz, dy, dx = array.shape
        if any(val == 1 for plane in w_max_cube for row in plane for val in row):
            array = numpy.concatenate((array, numpy.zeros((1, dz, dy, dx), dtype=int)), axis=0)
            dw, dz, dy, dx = array.shape
        if any(val == 1 for plane in z_min_cube for row in plane for val in row):
            array = numpy.concatenate((numpy.zeros((dw, 1, dy, dx), dtype=int), array), axis=1)
            dw, dz, dy, dx = array.shape
        if any(val == 1 for plane in z_max_cube for row in plane for val in row):
            array = numpy.concatenate((array, numpy.zeros((dw, 1, dy, dx), dtype=int)), axis=1)
            dw, dz, dy, dx = array.shape
        if any(val == 1 for plane in y_min_cube for row in plane for val in row):
            array = numpy.concatenate((numpy.zeros((dw, dz, 1, dx), dtype=int), array), axis=2)
            dw, dz, dy, dx = array.shape
        if any(val == 1 for plane in y_max_cube for row in plane for val in row):
            array = numpy.concatenate((array, numpy.zeros((dw, dz, 1, dx), dtype=int)), axis=2)
            dw, dz, dy, dx = array.shape
        if any(val == 1 for plane in x_min_cube for row in plane for val in row):
            array = numpy.concatenate((numpy.zeros((dw, dz, dy, 1), dtype=int), array), axis=3)
            dw, dz, dy, dx = array.shape
        if any(val == 1 for plane in x_max_cube for row in plane for val in row):
            array = numpy.concatenate((array, numpy.zeros((dw, dz, dy, 1), dtype=int)), axis=3)
        self._array = array

    def run_step(self) -> numpy.array:
        self.expand_space()
        copy = numpy.copy(self._array)
        dw, dz, dy, dx = self._array.shape
        for w in range(dw):
            for z in range(dz):
                for y in range(dy):
                    for x in range(dx):
                        point = copy[w, z, y, x]
                        active_neighbors = count_active_neighbors(copy, w, z, y, x)
                        if point == 1 and (active_neighbors < 2 or active_neighbors > 3):
                            self._array[w, z, y, x] = 0
                        if point == 0 and active_neighbors == 3:
                            self._array[w, z, y, x] = 1


def run_sim(space: Space) -> int:
    sim_step = 0
    while sim_step < 6:
        sim_step += 1
        space.run_step()
        print(f"Completed step {sim_step}")
    res = sum([sum([pos for pos in row]) for cube in space.array for surface in cube for row in surface])
    return res


test1 = """.#.
..#
###
"""

res = run_sim(Space(test1.splitlines()))
assert res == 848

with open("i17.txt", "r") as f:
    lines = f.readlines()
res = run_sim(Space(lines))
print(res)
