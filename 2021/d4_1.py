from typing import Optional
import numpy as np


class Board:
    def __init__(self, lines: list[str]) -> None:
        arr = None
        for line in lines:
            txt_nums = filter(None, line.strip().split(" ", ))
            nums = [int(n) for n in txt_nums]
            if arr is None:
                arr = np.empty((0, 5), dtype=int)
            arr = np.vstack((arr, nums))
        self._arr: np.ndarray = arr
        self._marked = np.zeros(arr.shape, bool)

    def mark_and_check(self, num: int) -> Optional[int]:
        index = np.where(self._arr == num)
        if len(index[0]) == 0:
            return None
        self._marked[index] = True
        if not (np.all(self._marked[index[0]]) or np.all(self._marked[:, index[1]])):
            return None
        non_marked = self._arr[np.invert(self._marked)]
        arr_sum = sum(non_marked)
        return num * arr_sum


def main() -> None:
    with open("i4.txt", "r") as i:
        lines = i.readlines()
    run(lines)


def run(lines: list[str]) -> None:
    numbers = [int(n) for n in lines[0].split(",")]
    board_lines: list[str] = []
    boards: list[Board] = []
    for line in lines[2:]:
        if not line or line == "\n":
            boards.append(Board(board_lines))
            board_lines = []
        else:
            board_lines.append(line)
    boards.append(Board(board_lines))

    for num in numbers:
        for board in boards:
            if res := board.mark_and_check(num):
                print(res)
                return


def test() -> None:
    txt = """7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"""
    run(txt.splitlines())


if __name__ == "__main__":
    test()
    main()
