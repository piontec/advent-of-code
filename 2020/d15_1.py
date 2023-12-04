from typing import Dict, List


def find_nth_num(input_nums: str, limit: int) -> int:
    nums = [int(x) for x in input_nums.split(",")]
    turn = 0
    num_counts: Dict[int, List[int]] = {}
    last = 0
    num = 0
    while turn <= limit:
        turn += 1
        last = num
        if turn <= len(nums):
            num = nums[turn - 1]
        else:
            num = 0 if len(num_counts[last]) == 1 else num_counts[last][-1] - num_counts[last][-2]
        if num not in num_counts.keys():
            num_counts[num] = []
        num_counts[num].append(turn)
    return last


res1 = find_nth_num("0,3,6", 2020)
assert res1 == 436

res2 = find_nth_num("1,3,2", 2020)
assert res2 == 1

res = find_nth_num("0,14,6,20,1,4", 2020)
print(res)
