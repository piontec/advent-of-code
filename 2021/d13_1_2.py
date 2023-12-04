from typing import Tuple

import numpy as np

Point = Tuple[int, int]
Fold = Tuple[str, int]


def main() -> None:
    with open("i13.txt", "r") as i:
        lines = i.readlines()
    res = run(lines)
    print(res)


def do_fold(paper: np.ndarray, fold: Fold) -> np.ndarray:
    if fold[0] == "x":
        fx = fold[1]
        assert fx == (paper.shape[1] - 1) // 2
        for y in range(0, paper.shape[0]):
            for x in range(fx + 1, paper.shape[1]):
                if paper[y, x] == 1:
                    paper[y, fx - (x - fx)] = 1
        return np.delete(paper, np.s_[fx:], axis=1)
    else:
        fy = fold[1]
        assert fy == (paper.shape[0] - 1) // 2
        for y in range(fy + 1, paper.shape[0]):
            for x in range(0, paper.shape[1]):
                if paper[y, x] == 1:
                    paper[fy - (y - fy), x] = 1
        return np.delete(paper, np.s_[fy:], axis=0)


def run(lines: list[str], just_one: bool = False) -> int:
    dots: list[Point] = []
    folds: list[Fold] = []
    in_folds = False
    for line in lines:
        if not line.strip():
            in_folds = True
            continue
        if not in_folds:
            x, y = [int(i) for i in line.strip().split(",")]
            dots.append((x, y))
        else:
            fold_expr = line.strip().split(" ")[2]
            axis, val = fold_expr.split("=")
            folds.append((axis, int(val)))
    max_x = max(d[0] for d in dots)
    max_y = max(d[1] for d in dots)
    paper = np.zeros(shape=(max_y + 1, max_x + 1), dtype=np.int8)
    for dot in dots:
        paper[dot[1], dot[0]] = 1
    for fold in folds:
        paper = do_fold(paper, fold)
        if just_one:
            break  # do only first one
    return sum(paper[paper == 1])


def test() -> None:
    txt = """6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"""
    assert run(txt.splitlines(), just_one=True) == 17


if __name__ == "__main__":
    test()
    main()
