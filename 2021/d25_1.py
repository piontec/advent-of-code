from typing import Tuple

import numpy as np



def main() -> None:
    with open("i25.txt", "r") as i:
        lines = i.readlines()
    res = run(lines)
    print(res)


def run(lines: list[str]) -> int:
    tcm = []
    for line in lines:
        tcm.append([c for c in line.strip()])
    cm = np.array(tcm)
    tcm = None
    changed = True
    rounds = 0
    while changed:
        changed = False
        rounds += 1
        ncm = np.full(cm.shape, fill_value=".")

        for y in range(cm.shape[0]):
            for x in range(cm.shape[1]):
                if cm[y, x] == ">":
                    if cm[y, (x + 1) % cm.shape[1]] == ".":
                        ncm[y, (x + 1) % cm.shape[1]] = ">"
                        changed = True
                    else:
                        ncm[y, x] = ">"
        for y in range(cm.shape[0]):
            for x in range(cm.shape[1]):
                if cm[y, x] == "v":
                    y_next = (y + 1) % cm.shape[0]
                    if cm[y_next, x] != "v" and ncm[y_next, x] != ">":
                        ncm[(y + 1) % cm.shape[0], x] = "v"
                        changed = True
                    else:
                        ncm[y, x] = "v"
        cm = ncm
    return rounds


def test() -> None:
    txt = """v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>"""
    assert run(txt.splitlines()) == 58


if __name__ == "__main__":
    test()
    main()
