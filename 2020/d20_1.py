import math
from typing import List, NewType, Dict, Optional

import numpy as np

Direction = NewType('Direction', str)
Left = Direction("left")
Right = Direction("right")
Up = Direction("up")
Down = Direction("down")


class Tile:
    @staticmethod
    def from_lines(tid: int, input_lines: List[str]) -> "Tile":
        array = [[0 if e == "." else 1 for e in line] for line in input_lines]
        np_array = np.array(array)
        return Tile(tid, np_array)

    def __init__(self, tid: int, array: np.array):
        self.tid = tid
        self.array = array
        self.neighbors: Dict[Direction, Optional["Tile"]] = {}

    def __repr__(self) -> str:
        return str(self.tid)

    def get_all_options(self) -> List[np.array]:
        arr_copy = np.copy(self.array)
        arr_copy_flipped = np.fliplr(np.copy(arr_copy))
        options: List[np.array] = [np.copy(arr_copy), np.copy(arr_copy_flipped)]
        for arr in [arr_copy, arr_copy_flipped]:
            for rot in range(1, 4):
                arr = np.rot90(arr)
                options.append(np.copy(arr))
        return options

    def matches(self, tile2: "Tile") -> (bool, Optional[Direction]):
        options = tile2.get_all_options()
        for t_opt in options:
            match = False
            matched_dir = None
            for direction in [Left, Up, Right, Down]:
                if direction == Left:
                    if np.array_equal(self.array[:, 0], t_opt[:, -1]):
                        match = True
                elif direction == Up:
                    if np.array_equal(self.array[0, :], t_opt[-1, :]):
                        match = True
                elif direction == Right:
                    if np.array_equal(self.array[:, -1], t_opt[:, 0]):
                        match = True
                elif direction == Down:
                    if np.array_equal(self.array[-1, :], t_opt[0, :]):
                        match = True
                if match:
                    matched_dir = direction
                    break
            if match:
                tile2.array = t_opt
                return True, matched_dir
        return False, None


class Board:
    def __init__(self, tiles: List[Tile]):
        self.tiles = tiles
        self.size = int(math.sqrt(len(tiles)))
        self.area = np.zeros((self.size, self.size))

    def solve(self) -> None:
        to_visit: List[Tile] = [self.tiles[0]]
        visited: List[Tile] = []
        while len(to_visit) > 0:
            tile = to_visit.pop()
            visited.append(tile)
            for tile2 in self.tiles:
                if tile == tile2:
                    continue
                match, direction = tile.matches(tile2)
                if match:
                    if direction in tile.neighbors.keys():
                        raise Exception("was setting this neighbor already")
                    tile.neighbors[direction] = tile2
                    if tile2 not in visited and tile2 not in to_visit:
                        to_visit.append(tile2)
        self.assign_result()

    @property
    def answer(self) -> int:
        corners_multi = self.area[0, 0] * self.area[0, self.size - 1] * self.area[self.size - 1, 0] * self.area[
            self.size - 1, self.size - 1]
        return int(corners_multi)

    def assign_result(self):
        tile = self.tiles[0]
        while Left in tile.neighbors.keys():
            tile = tile.neighbors[Left]
        while Up in tile.neighbors.keys():
            tile = tile.neighbors[Up]
        first_in_row = tile
        for y in range(self.size):
            for x in range(self.size):
                self.area[y][x] = int(tile.tid)
                if Right in tile.neighbors:
                    tile = tile.neighbors[Right]
            if Down in first_in_row.neighbors:
                first_in_row = first_in_row.neighbors[Down]
                tile = first_in_row


def parse(input_lines: List[str]) -> Board:
    tiles: List[Tile] = []
    tile_lines: List[str] = []
    tile_id = 0
    for line in input_lines:
        if line.startswith("Tile "):
            tile_id = int(line.strip(":\n").split(" ")[1])
        elif line == "" or line == "\n":
            tile = Tile.from_lines(tile_id, tile_lines)
            tiles.append(tile)
            tile_lines = []
        else:
            tile_lines.append(line.strip())
    if len(tile_lines) > 0:
        tile = Tile.from_lines(tile_id, tile_lines)
        tiles.append(tile)
    return Board(tiles)


def solve(input_lines: List[str]) -> int:
    board = parse(input_lines)
    board.solve()
    return board.answer


test1 = """Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...
"""

# res1 = solve(test1.splitlines())
# assert res1 == 20899048083289

with open("i20.txt", "r") as f:
    lines = f.readlines()
res = solve(lines)
print(res)
