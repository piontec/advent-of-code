from typing import List

def count_options(low: int, high: int) -> List[int]:
    res: List[int] = []
    for num in range(low, high + 1):
        digits = [int(d) for d in str(num)]
        found_double = False
        decreases = False
        for i in range(1, len(digits)):
            if digits[i] < digits[i - 1]:
                decreases = True
                break
            if digits[i] == digits[i - 1]:
                found_double = True
        if not decreases and found_double:
            res.append(num)
    return res

print(str.format("P1: {}", len(count_options(178416, 676461))))