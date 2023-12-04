from typing import Dict, List


def find_nth_num(input_nums: str, limit: int) -> int:
    nums = [int(x) for x in input_nums.split(",")]
    turn = 0
    num_counts: Dict[int, List[int]] = {}
    last = 0
    num = 0
    while turn <= limit:
        turn += 1
        if turn % 100000 == 0:
            print(f"Turn: {turn}")
        last = num
        if turn <= len(nums):
            num = nums[turn - 1]
        else:
            num = 0 if len(num_counts[last]) == 1 else num_counts[last][-1] - num_counts[last][-2]
        if num not in num_counts.keys():
            num_counts[num] = []
        num_counts[num].append(turn)
        if len(num_counts[num]) > 2:
            num_counts[num] = num_counts[num][-2:]
    return last


# res1 = find_nth_num("0,3,6", 30000000)
# assert res1 == 175594

res = find_nth_num("0,14,6,20,1,4", 30000000)
print(res)
