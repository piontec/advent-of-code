import collections


def main() -> None:
    with open("i6.txt", "r") as i:
        lines = i.readlines()
    res = run(lines[0])
    print(res)
    res = run(lines[0], 256)
    print(res)


def run(line: str, how_long: int = 80) -> int:
    days = [int(n) for n in line.split(",")]
    fishes = dict(collections.Counter(days))
    for i in range(9):
        if i not in fishes:
            fishes[i] = 0
    for d in range(how_long):
        zeros = fishes[0]
        for v in range(8):
            fishes[v] = fishes[v + 1]
        fishes[6] += zeros
        fishes[8] = zeros
    res = sum(fishes.values())
    return res


def test() -> None:
    txt = "3,4,3,1,2"
    assert run(txt, 18) == 26
    assert run(txt) == 5934


if __name__ == "__main__":
    test()
    main()
