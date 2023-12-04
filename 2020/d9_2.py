from typing import List

magic_number = 23278925


def check_list(str_list: List[str], to_find: int) -> int:
    num_list = [int(x.strip()) for x in str_list]
    upper_limit = len(num_list)
    found_range = None
    for i in range(upper_limit - 1):
        current_sum = num_list[i]
        for j in range(i + 1, upper_limit):
            current_sum += num_list[j]
            if current_sum == to_find:
                found_range = (i, j)
                break
            if current_sum > to_find:
                break
        if found_range:
            break
    if not found_range:
        raise Exception("not found")
    l_min = min(num_list[found_range[0]:found_range[1] + 1])
    l_max = max(num_list[found_range[0]:found_range[1] + 1])
    return l_min + l_max


test1 = """35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
"""

res = check_list(test1.splitlines(), 127)
assert res == 62

with open("i9.txt", "r") as f:
    lines = f.readlines()
res = check_list(lines, magic_number)
print(res)
