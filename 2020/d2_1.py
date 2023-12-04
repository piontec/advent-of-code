import sys

with open("i2.txt") as f:
    lines = f.readlines()

valid = 0
for line in lines:
    meta, passwd = line.split(":", maxsplit=1)
    passwd = passwd.strip()
    range, char = meta.split(" ")
    low_str, high_str = range.split("-")
    low, high = int(low_str), int(high_str)
    char_count = sum(c == char for c in passwd)
    if low <= char_count <= high:
        valid += 1

print(valid)
