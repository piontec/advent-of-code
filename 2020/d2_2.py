with open("i2.txt") as f:
    lines = f.readlines()

valid = 0
for line in lines:
    meta, passwd = line.split(":", maxsplit=1)
    passwd = passwd.strip()
    range_str, char = meta.split(" ")
    low_str, high_str = range_str.split("-")
    low, high = int(low_str), int(high_str)
    if (passwd[low - 1] == char and passwd[high - 1] != char) or (passwd[low - 1] != char and passwd[high - 1] == char):
        valid += 1

print(valid)
