from typing import Tuple, List

import numpy as np

Point = Tuple[int, int]


def main() -> None:
    with open("i11.txt", "r") as i:
        lines = i.readlines()
    res = run(lines)
    print(res)


def get_neighbor_points(x: int, y: int, shape: Tuple[int, int]) -> List[Point]:
    lp: list[Point] = []
    for ny in range(y - 1, y + 2):
        for nx in range(x - 1, x + 2):
            if nx == x and ny == y:
                continue
            if nx < 0 or nx > shape[1] - 1 or ny < 0 or ny > shape[0] - 1:
                continue
            lp.append((ny, nx))
    return lp


def step(arr: np.ndarray) -> int:
    flashed = np.zeros(shape=arr.shape, dtype=np.int8)
    arr += 1
    for y in range(arr.shape[0]):
        for x in range(arr.shape[1]):

            def flash_it(fy: int, fx: int):
                if arr[fy, fx] <= 9:
                    return
                if flashed[fy, fx]:
                    return
                flashed[fy, fx] = 1
                neighbors = get_neighbor_points(fx, fy, arr.shape)
                for n in neighbors:
                    arr[n] += 1
                for n in neighbors:
                    flash_it(n[0], n[1])

            flash_it(y, x)
    arr[flashed > 0] = 0
    return sum(flashed[flashed > 0])


def run(lines: list[str]) -> int:
    arr = np.ndarray((10, 10), dtype=np.int8)
    row = 0
    for line in lines:
        arr[row] = [int(i) for i in line.strip()]
        row += 1
    step_count = 0
    while True:
        flsh = step(arr)
        step_count += 1
        if flsh == 100:
            break
    return step_count


def test() -> None:
    txt = """5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"""
    assert run(txt.splitlines()) == 195


if __name__ == "__main__":
    test()
    main()
