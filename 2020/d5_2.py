from typing import Tuple


def bin_find(low, high, code):
    for c in code:
        mid = int((high + low + 1) / 2)
        if c == "F" or c == "L":
            high = mid - 1
        else:
            low = mid
    return low


def get_seat_rc(code: str) -> Tuple[int, int]:
    fb = code[:7]
    lr = code[7:]
    row = bin_find(0, 127, fb)
    col = bin_find(0, 7, lr)
    return row, col


with open("i5.txt", "r") as f:
    lines = f.readlines()

all_seats = [(r, c) for r in range(128) for c in range(8)]
for line in lines:
    rc = get_seat_rc(line.strip())
    all_seats.remove(rc)
for r in range(128):
    full_row = [(r, c) for c in range(8)]
    if all(rc in all_seats for rc in full_row):
        all_seats = all_seats[8:]
    else:
        break
first_row = all_seats[0][0]
my_seat = None
for rc in all_seats:
    if rc[0] != first_row:
        my_seat = rc
        break
print(my_seat[0] * 8 + my_seat[1])
