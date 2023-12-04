from typing import Tuple

import numpy as np

Point = Tuple[int, int]


def main() -> None:
    with open("i9.txt", "r") as i:
        lines = i.readlines()
    res = run(lines)
    print(res)


def get_neighbors (arr: np.ndarray, p: Point) -> list[Point]:
    neighbors: list[Point] = []
    y, x = p
    if y > 0:
        neighbors.append((y - 1, x))
    if y < arr.shape[0] - 1:
        neighbors.append((y + 1, x))
    if x > 0:
        neighbors.append((y, x - 1))
    if x < arr.shape[1] - 1:
        neighbors.append((y, x + 1))
    return neighbors


def local_min(arr: np.ndarray, p: Point):
    val = arr[p]
    return all(arr[p] > val for p in get_neighbors(arr, p))


def run(lines: list[str]) -> int:
    risk_sum = 0
    dim_x = len(lines[0].strip())
    dim_y = len(lines)
    arr = np.zeros((dim_y, dim_x), dtype=np.uint8)
    for i in range(len(lines)):
        arr[i] = [int(x) for x in lines[i].strip()]
    for y in range(dim_y):
        for x in range(dim_x):
            if local_min(arr, (y, x)):
                risk_sum += arr[y, x] + 1
    return risk_sum


def test() -> None:
    txt = """2199943210
3987894921
9856789892
8767896789
9899965678"""
    assert run(txt.splitlines()) == 15


if __name__ == "__main__":
    test()
    main()
