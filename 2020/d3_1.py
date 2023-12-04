from dataclasses import dataclass


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


with open("i3.txt") as f:
    lines = f.readlines()

terrain = [list(line.strip()) for line in lines]
terrain_width = len(terrain[0])
height = len(terrain)
my_pos = Position(Pos(0, 0), Pos(0, 0))
movement = Pos(3, 1)
trees = 0
while my_pos.abs.y < height - 1:
    my_pos = next_pos(terrain_width, my_pos, movement)
    if terrain[my_pos.rel.y][my_pos.rel.x] == "#":
        trees += 1
print(trees)
