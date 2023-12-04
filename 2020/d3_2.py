from dataclasses import dataclass
from typing import List


@dataclass
class Pos:
    x: int
    y: int


@dataclass
class Position:
    rel: Pos
    abs: Pos


def next_pos(width: int, current_pos: Position, move: Pos) -> Position:
    return Position(rel=Pos((current_pos.rel.x + move.x) % width, current_pos.rel.y + move.y),
                    abs=Pos(current_pos.abs.x + move.x, current_pos.abs.y + move.y))


def count_trees(terrain: List[List[int]], move: Pos) -> int:
    trees = 0
    terrain_height = len(terrain)
    terrain_width = len(terrain[0])
    my_pos = Position(Pos(0, 0), Pos(0, 0))
    while my_pos.abs.y < terrain_height - 1:
        my_pos = next_pos(terrain_width, my_pos, move)
        if terrain[my_pos.rel.y][my_pos.rel.x] == "#":
            trees += 1
    return trees


with open("i3.txt") as f:
    lines = f.readlines()

ter = [list(line.strip()) for line in lines]
t1 = count_trees(ter, Pos(1, 1))
t2 = count_trees(ter, Pos(3, 1))
t3 = count_trees(ter, Pos(5, 1))
t4 = count_trees(ter, Pos(7, 1))
t5 = count_trees(ter, Pos(1, 2))
print(t1 * t2 * t3 * t4 * t5)
