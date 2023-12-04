from typing import Set, List


def unique_for_group(answers: str) -> str:
    res: Set[str] = set()
    for c in answers:
        res.add(c)
    return "".join(res)


def count_for_input(input_lines: List[str]) -> int:
    combined = ""
    group: List[str] = []
    group_counts: List[int] = []
    for line in input_lines:
        if line == "\n" or line == "":
            unique = unique_for_group(combined)
            found_in_all = sum(all(c in g for g in group) for c in unique)
            group_counts.append(found_in_all)
            combined = ""
            group = []
        else:
            combined += line.strip()
            group.append(line.strip())
    return sum(group_counts)


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

b

"""
assert count_for_input(test_input.splitlines()) == 6

with open("i6.txt", "r") as f:
    lines = f.readlines()
    lines.append("\n")

print(count_for_input(lines))
