def bin_find(low, high, code):
    for c in code:
        mid = int((high + low + 1) / 2)
        if c == "F" or c == "L":
            high = mid - 1
        else:
            low = mid
    return low


def get_seat_id(code: str) -> int:
    fb = code[:7]
    lr = code[7:]
    row = bin_find(0, 127, fb)
    col = bin_find(0, 7, lr)
    return row * 8 + col


assert get_seat_id("FBFBBFFRLR") == 357
assert get_seat_id("BFFFBBFRRR") == 567

with open("i5.txt", "r") as f:
    lines = f.readlines()

highest = max(get_seat_id(line.strip()) for line in lines)
print(highest)
