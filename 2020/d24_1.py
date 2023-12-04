from typing import List, Tuple, Dict

# loc -> is_black
Tiles = Dict[Tuple[int, int], bool]
Pos = Tuple[int, int]


def move_pos(pos: Pos, line: str) -> (Pos, str):
    if line.startswith("se"):
        line = line[2:]
        pos = (pos[0] - 3, pos[1] + 2)
    elif line.startswith("sw"):
        line = line[2:]
        pos = (pos[0] - 3, pos[1] - 2)
    elif line.startswith("ne"):
        line = line[2:]
        pos = (pos[0] + 3, pos[1] + 2)
    elif line.startswith("nw"):
        line = line[2:]
        pos = (pos[0] + 3, pos[1] - 2)
    elif line.startswith("e"):
        line = line[1:]
        pos = (pos[0], pos[1] + 4)
    elif line.startswith("w"):
        line = line[1:]
        pos = (pos[0], pos[1] - 4)
    return pos, line


def parse_line(line: str, tiles: Tiles):
    pos: Pos = (0, 0)
    while len(line) > 0:
        pos, line = move_pos(pos, line)
    if pos in tiles.keys():
        tiles[pos] = not tiles[pos]
    else:
        tiles[pos] = True


def run(lines: List[str]) -> int:
    tiles: Tiles = {}
    for line in lines:
        parse_line(line.strip(), tiles)
    black_count = sum(1 if tiles[k] else 0 for k in tiles.keys())
    return black_count


t0 = {}
parse_line("nwwswee", t0)
assert t0[(0, 0)] is True
test1 = """sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew
"""

res1 = run(test1.splitlines())
assert res1 == 10

with open("i24.txt", "r") as f:
    lines = f.readlines()
res = run(lines)
print(res)
