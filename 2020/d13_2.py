from typing import List, Dict

import numpy as np


def check_timetable(lines: List[str]) -> int:
    bus_offsets: Dict[int, int] = {}
    buses: List[int] = []
    offset = -1
    for num in lines[0].split(","):
        offset += 1
        if num == "x":
            continue
        line_num = int(num)
        buses.append(line_num)
        bus_offsets[line_num] = offset

    timestamp = 0
    jump = buses[0]
    for line_num_idx in range(len(buses) - 1):
        offset_diff = bus_offsets[buses[line_num_idx + 1]] - bus_offsets[buses[line_num_idx]]
        while True:
            timestamp += jump
            if (timestamp + offset_diff) % buses[line_num_idx + 1] != 0:
                continue
            timestamp += offset_diff
            jump = np.lcm(jump, buses[line_num_idx + 1])
            break
    return timestamp - bus_offsets[buses[-1]]


res0 = check_timetable(["3,8,x,5"])
assert res0 == 87

test1 = """7,13,x,x,59,x,31,19
"""
res1 = check_timetable(test1.splitlines())
assert res1 == 1068781

res2 = check_timetable(["67,7,59,61"])
assert res2 == 754018

res3 = check_timetable(["1789,37,47,1889"])
assert res3 == 1202161486

with open("i13.txt", "r") as f:
    lines = f.readlines()
res = check_timetable(lines[1:])
print(res)
