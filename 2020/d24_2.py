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


def sim_round(tiles: Tiles) -> Tiles:
    new_floor: Tiles = {}
    added = check_tiles(tiles, tiles, new_floor, return_added=True)
    tiles.update(added)
    check_tiles(added, tiles, new_floor, return_added=False)
    return new_floor


def check_tiles(tiles_to_check: Tiles, old_floor: Tiles, new_floor: Tiles, return_added: bool) -> Tiles:
    to_add: Tiles = {}
    for tile_pos in tiles_to_check.keys():
        blacks_around = 0
        for neigh_dir in "se", "sw", "w", "nw", "ne", "e":
            neigh_pos, _ = move_pos(tile_pos, neigh_dir)
            if neigh_pos in old_floor.keys() and old_floor[neigh_pos]:
                blacks_around += 1
            if neigh_pos not in old_floor.keys():
                if return_added:
                    to_add[neigh_pos] = False
            if tiles_to_check[tile_pos] and (blacks_around == 0 or blacks_around > 2):
                new_floor[tile_pos] = False
            elif not tiles_to_check[tile_pos] and blacks_around == 2:
                new_floor[tile_pos] = True
            else:
                new_floor[tile_pos] = tiles_to_check[tile_pos]
    return to_add


def run(lines: List[str]) -> int:
    tiles: Tiles = {}
    for line in lines:
        parse_line(line.strip(), tiles)
    for i in range(1, 101):
        tiles = sim_round(tiles)
        black_count = sum(1 if tiles[k] else 0 for k in tiles.keys())
        print(f"Day {i}: {black_count}")
    return black_count


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
assert res1 == 2208

with open("i24.txt", "r") as f:
    lines = f.readlines()
res = run(lines)
print(res)
