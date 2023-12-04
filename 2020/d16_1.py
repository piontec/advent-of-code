from typing import List, Tuple


class Limit:
    def __init__(self, line: str):
        self.name, ranges = line.strip().split(":")
        self.ranges: List[Tuple[int, int]] = []
        for r in ranges.split("or"):
            low, high = r.strip().split("-")
            self.ranges.append((int(low), int(high)))


def validate_tickets(lines: List[str]) -> int:
    limits: List[Limit] = []
    idx = 0
    while True:
        if lines[idx] in ["\n", ""]:
            break
        limits.append(Limit(lines[idx]))
        idx += 1
    my_ticket = [int(x) for x in lines[idx + 2].strip().split(",")]
    idx += 5
    tickets: List[List[int]] = []
    for i in range(idx, len(lines)):
        ticket = [int(x) for x in lines[i].strip().split(",")]
        tickets.append(ticket)
    # parsing done, run logic
    bad_values: List[int] = []
    for ticket in tickets:
        for num in ticket:
            found = False
            for limit in limits:
                for valid_range in limit.ranges:
                    if valid_range[0] <= num <= valid_range[1]:
                        found = True
                        break
                if found:
                    break
            else:
                bad_values.append(num)
    return sum(bad_values)


test1 = """class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
"""
res1 = validate_tickets(test1.splitlines())
assert res1 == 71

with open("i16.txt", "r") as f:
    lines = f.readlines()
res = validate_tickets(lines)
print(res)
