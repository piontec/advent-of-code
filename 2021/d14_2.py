import math


def main() -> None:
    with open("i14.txt", "r") as i:
        lines = i.readlines()
    res = run(lines)
    print(res)


def count_symbols(pairs: dict[str, int], pair: str, count: int = 1) -> None:
    if pair not in pairs:
        pairs[pair] = count
    else:
        pairs[pair] += count


def run(lines: list[str], steps: int = 40) -> int:
    poly = lines[0].strip()
    pairs: dict[str, int] = {}
    for i in range(len(poly) - 1):
        pair = poly[i:i + 2]
        count_symbols(pairs, pair)
    rules: dict[str, str] = {}
    for line in lines[2:]:
        left, right = line[0:2], line[6]
        rules[left] = right

    for i in range(steps):
        new_pairs: dict[str, int] = {}
        for pair in pairs:
            insert = rules[pair]
            count = pairs[pair]
            first, second = pair[0] + insert, insert + pair[1]
            count_symbols(new_pairs, first, count)
            count_symbols(new_pairs, second, count)
        pairs = new_pairs

    symbols: dict[str, int] = {}
    for pair in pairs:
        count_symbols(symbols, pair[0], pairs[pair])
        count_symbols(symbols, pair[1], pairs[pair])
    max_freq = math.ceil(max(symbols.values()) / 2)
    min_freq = math.ceil(min(symbols.values()) / 2)
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
    assert run(txt.splitlines()) == 2188189693529


if __name__ == "__main__":
    test()
    main()
