from typing import List, Dict


def check_timetable(lines: List[str]) -> int:
    timestamp = int(lines[0])
    bus_ids = [int(i) for i in lines[1].strip().split(",") if i != "x"]
    departures: Dict[int, int] = {}
    for i in bus_ids:
        offset = i - timestamp % i
        departures[offset] = i
    min_offset = min(departures.keys())
    res = min_offset * departures[min_offset]
    return res


test1 = """939
7,13,x,x,59,x,31,19
"""

res1 = check_timetable(test1.splitlines())
assert res1 == 295

with open("i13.txt", "r") as f:
    lines = f.readlines()
res = check_timetable(lines)
print(res)
