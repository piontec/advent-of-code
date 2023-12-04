from collections import Counter


def main() -> None:
    with open("i14.txt", "r") as i:
        lines = i.readlines()
    res = run(lines)
    print(res)


def step(poly: str, rules: dict[str, str]) -> str:
    left = poly[0]
    res = left
    for right in poly[1:]:
        insert = rules[left + right]
        res += insert + right
        left = right
    return res


def run(lines: list[str], steps: int = 10) -> int:
    poly = lines[0].strip()
    rules: dict[str, str] = {}
    for line in lines[2:]:
        left, right = line[0:2], line[6]
        rules[left] = right

    for i in range(steps):
        poly = step(poly, rules)
        print(i)

    freq = Counter([c for c in poly])
    max_freq = max(freq.values())
    min_freq = min(freq.values())
    return max_freq - min_freq


def test() -> None:
    txt = """NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"""
    assert run(txt.splitlines()) == 1588


if __name__ == "__main__":
    test()
    main()
