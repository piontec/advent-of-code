from dataclasses import dataclass
from typing import Tuple

import numpy as np

Point = Tuple[int, int]
Fold = Tuple[str, int]

Range = Tuple[int, int]


@dataclass
class Cuboid:
    on: bool
    x: Range
    y: Range
    z: Range


def main() -> None:
    with open("i22.txt", "r") as i:
        lines = i.readlines()
    res = run(lines)
    print(res)


def run(lines: list[str], size: int = 101) -> int:
    cuboids: list[Cuboid] = []
    for line in lines:
        on_txt, rest = line.strip().split(" ")
        on = on_txt == "on"
        ranges = rest.split(",")
        x_nums = ranges[0][2:].split(".")
        x_min, x_max = int(x_nums[0]), int(x_nums[2])
        y_nums = ranges[1][2:].split(".")
        y_min, y_max = int(y_nums[0]), int(y_nums[2])
        z_nums = ranges[2][2:].split(".")
        z_min, z_max = int(z_nums[0]), int(z_nums[2])
        c = Cuboid(on, (x_min, x_max), (y_min, y_max), (z_min, z_max))
        cuboids.append(c)
    cube = np.zeros((size, size, size), dtype=np.int8)
    offset = 50
    range_min = - size // 2
    range_max = size // 2
    for c in cuboids:
        if (c.x[0] < range_min and c.x[1] < range_min) \
                or (c.x[0] > range_max and c.x[1] > range_max) \
                or (c.y[0] < range_min and c.y[1] < range_min) \
                or (c.y[0] > range_max and c.y[1] > range_max) \
                or (c.z[0] < range_min and c.z[1] < range_min) \
                or (c.z[0] > range_max and c.z[1] > range_max):
            continue
        cube[c.z[0] + offset:c.z[1] + offset + 1, c.y[0] + offset:c.y[1] + offset + 1,
            c.x[0] + offset:c.x[1] + offset + 1] = 1 if c.on else 0
    res = sum(cube[cube == 1])
    return res


def test() -> None:
    txt = """on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682"""
    assert run(txt.splitlines()) == 590784


if __name__ == "__main__":
    test()
    main()
