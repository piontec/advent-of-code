from typing import List, Dict


def count_jolts(lines: List[str]) -> int:
    num_lines = [int(x.strip()) for x in lines]
    num_lines.append(0)
    num_lines.append(max(num_lines) + 3)
    num_lines.sort()
    jolts: Dict[int, int] = {0: 0, 1: 0, 2: 0, 3: 0}
    for i in range(len(num_lines) - 1):
        diff = num_lines[i + 1] - num_lines[i]
        jolts[diff] += 1
    return jolts[1] * jolts[3]


test1 = """16
10
15
5
1
11
7
19
6
12
4
"""

res = count_jolts(test1.splitlines())
assert res == 35

with open("i10.txt", "r") as f:
    lines = f.readlines()
res = count_jolts(lines)
print(res)
