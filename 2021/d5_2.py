from typing import Optional
import numpy as np


def main() -> None:
    with open("i5.txt", "r") as i:
        lines = i.readlines()
    run(lines, 1000)


def run(lines: list[str], dimension: int) -> None:
    arr = np.zeros((dimension, dimension), dtype=int)
    for line in lines:
        txt_start, _, txt_end = line.split(" ")
        start = [int(n) for n in txt_start.split(",")]
        end = [int(n) for n in txt_end.split(",")]
        if start[0] == end[0]:  # vertical
            if start[1] > end[1]:
                start[1], end[1] = end[1], start[1]
            for y in range(start[1], end[1] + 1):
                arr[start[0], y] += 1
        elif start[1] == end[1]:  # horizontal
            if start[0] > end[0]:
                start[0], end[0] = end[0], start[0]
            for x in range(start[0], end[0] + 1):
                arr[x, start[1]] += 1
        else:
            x_dir = 1 if start[0] < end[0] else -1
            y_dir = 1 if start[1] < end[1] else -1
            for i in range(0, abs(start[0]-end[0]) + 1):
                x = start[0] + i * x_dir
                y = start[1] + i * y_dir
                arr[x, y] += 1
    found = len(arr[arr >= 2])
    print(found)


def test() -> None:
    txt = """0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"""
    run(txt.splitlines(), 10)


if __name__ == "__main__":
    test()
    main()
