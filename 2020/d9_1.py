from typing import List


def binary_find(num_list: List[int], val: int):
    low = 0
    high = len(num_list) - 1
    while True:
        if high - low <= 1:
            return val in [num_list[low], num_list[high]]
        mid = (low + high) // 2
        if num_list[mid] >= val:
            high = mid
        elif num_list[mid] < val:
            low = mid


def are_components_in_list(num_list: List[int], val: int) -> bool:
    num_list.sort()
    for i in range(len(num_list)):
        if binary_find(num_list[:i] + num_list[i + 1:], val - num_list[i]):
            return True
    return False


def check_list(str_list: List[str], preamble_length: int) -> int:
    num_list = [int(x.strip()) for x in str_list]
    for i in range(len(num_list) - preamble_length - 1):
        if not are_components_in_list(num_list[i:i + preamble_length], num_list[i + preamble_length]):
            return num_list[i + preamble_length]
    raise Exception("not found")


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

res = check_list(test1.splitlines(), 5)
assert res == 127

with open("i9.txt", "r") as f:
    lines = f.readlines()
res = check_list(lines, 25)
print(res)