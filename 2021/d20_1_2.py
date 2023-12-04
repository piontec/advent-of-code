from typing import Tuple

Coord = Tuple[int, int]


def main() -> None:
    with open("i20.txt", "r") as i:
        lines = i.readlines()
    res = run(lines)
    print(res)


def get_pixel(py: int, px: int, img: dict[Coord, bool], lookup: str, def_val: bool) -> bool:
    val = 0
    pwr = 0
    for y in range(py + 1, py - 2, -1):
        for x in range(px + 1, px - 2, -1):
            p = (y, x)
            if p in img:
                if img[p]:
                    val += 2 ** pwr
            else:
                if def_val:
                    val += 2 ** pwr
            pwr += 1
    return lookup[val] == "#"


def run(lines: list[str], steps: int = 50) -> int:
    lookup = lines[0].strip()
    img: dict[Coord, bool] = {}
    y = 0
    for line in lines[2:]:
        x = 0
        for c in line.strip():
            if c == "#":
                img[(y, x)] = True
            else:
                img[(y, x)] = False
            x += 1
        y += 1
    def_val = False
    for i in range(steps):
        all_y = [p[0] for p in img.keys()]
        all_x = [p[1] for p in img.keys()]
        min_y, max_y = min(all_y), max(all_y)
        min_x, max_x = min(all_x), max(all_x)
        new_img: dict[Coord, bool] = {}
        for y in range(min_y - 1, max_y + 2):
            for x in range(min_x - 1, max_x + 2):
                new_pixel = get_pixel(y, x, img, lookup, def_val)
                if new_pixel:
                    new_img[(y, x)] = True
                else:
                    new_img[(y, x)] = False
        img = new_img
        print(f"step: {i}")
        # print_image(img)
        if def_val:
            def_val_index = -1
        else:
            def_val_index = 0
        def_val = lookup[def_val_index] == "#"
    return len([k for k in img if img[k]])


def print_image(img: dict[Coord, bool]) -> None:
    all_y = [p[0] for p in img.keys()]
    all_x = [p[1] for p in img.keys()]
    min_y, max_y = min(all_y), max(all_y)
    min_x, max_x = min(all_x), max(all_x)
    for y in range(min_y, max_y + 1):
        for x in range(min_x, max_x + 1):
            print("#" if (y, x) in img and img[(y, x)] else ".", end='')
        print("")
    print("")


def test() -> None:
    txt = """..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#
    
#..#.
#....
##..#
..#..
..###"""
    val = run(txt.splitlines())
    assert val == 3351


if __name__ == "__main__":
    test()
    main()
