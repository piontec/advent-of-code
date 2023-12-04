from typing import Set, List


def count_for_group(answers: str) -> int:
    res: Set[str] = set()
    for c in answers:
        res.add(c)
    return len(res)


def count_for_input(input_lines: List[str]) -> int:
    combined = ""
    groups: List[int] = []
    for line in input_lines:
        if line == "\n" or line == "":
            groups.append(count_for_group(combined))
            combined = ""
        else:
            combined += line.strip()
    groups.append(count_for_group(combined))
    return sum(groups)


test_input = """abc

a
b
c

ab
ac

a
a
a
a

b"""
assert count_for_input(test_input.splitlines()) == 11

with open("i6.txt", "r") as f:
    lines = f.readlines()

print(count_for_input(lines))
