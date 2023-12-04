import copy
from typing import List, Tuple, Dict


class Limit:
    def __init__(self, line: str):
        name, ranges = line.strip().split(":")
        self.name: str = name
        self.ranges: List[Tuple[int, int]] = []
        for r in ranges.split("or"):
            low, high = r.strip().split("-")
            self.ranges.append((int(low), int(high)))


def is_valid(ticket: List[int], limits: List[Limit]) -> bool:
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
            return False
    return True


def find_possible_fields(col: List[int], limits: List[Limit]) -> List[str]:
    possible_fields: List[str] = []
    for limit in limits:
        if all(
                [(limit.ranges[0][0] <= val <= limit.ranges[0][1] or limit.ranges[1][0] <= val <= limit.ranges[1][1])
                 for
                 val in col]):
            possible_fields.append(limit.name)
    if len(possible_fields) == 0:
        raise Exception("no limit found")
    return possible_fields


def resolve_possible_fields(options: Dict[int, List[str]]) -> Dict[str, int]:
    final: Dict[str, int] = {}
    while len(options) > 0:
        single_match_cols = [col_num for col_num in options.keys() if len(options[col_num]) == 1]
        for col_num in single_match_cols:
            field_name = options[col_num][0]
            final[field_name] = col_num
            del(options[col_num])
            for opt_list in options.values():
                if field_name in opt_list:
                    opt_list.remove(field_name)
    return final


def match_fields(valid_tickets: List[List[int]], limits: List[Limit]) -> Dict[str, int]:
    matches: Dict[int, List[str]] = {}
    for col_id in range(len(valid_tickets[0])):
        col = [t[col_id] for t in valid_tickets]
        possible_fields = find_possible_fields(col, limits)
        matches[col_id] = possible_fields
    res = resolve_possible_fields(matches)
    return res


def validate_tickets(lines: List[str]) -> Dict[str, int]:
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
    valid_tickets = [t for t in tickets if is_valid(t, limits)]
    matched_fields = match_fields(valid_tickets, copy.deepcopy(limits))
    res = {lim.name: my_ticket[matched_fields[lim.name]] for lim in limits}
    return res


test1 = """class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9
"""

res1 = validate_tickets(test1.splitlines())
assert res1["class"] == 12
assert res1["row"] == 11
assert res1["seat"] == 13

with open("i16.txt", "r") as f:
    lines = f.readlines()
ticket = validate_tickets(lines)
res = 1
for k, v in ticket.items():
    if not k.startswith("departure"):
        continue
    res *= v
print(res)
