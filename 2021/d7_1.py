from typing import Tuple


def main() -> None:
    with open("i7.txt", "r") as i:
        lines = i.readlines()
    res = run(lines[0])
    print(res)


def run(line: str) -> Tuple[int, int]:
    crabs = [int(n) for n in line.split(",")]
    min_pos = min(crabs)
    max_pos = max(crabs)
    costs: dict[int, int] = {}
    for i in range(min_pos, max_pos + 1):
        costs[i] = sum(abs(i - c) for c in crabs)
    min_dist = min(costs.items(), key=lambda e: e[1])
    return min_dist


def test() -> None:
    txt = "16,1,2,0,4,2,7,1,2,14"
    assert run(txt) == (2, 37)


if __name__ == "__main__":
    test()
    main()
