import sys
import string

def remove_reduce(letter, poly):
    p = poly.replace(letter, "")
    p = p.replace(letter.upper(), "")
    return reduce(list(p))


def reduce(poly):
    while True:
        reacted = False
        for i in range(1, len(poly)):
            if poly[i-1].upper() == poly[i].upper() and ((poly[i-1].islower() and not poly[i].islower()) or (not poly[i-1].islower() and poly[i].islower())):
                poly = poly[:i-1]+poly[i+1:]
                reacted = True
                break
        if not reacted:
            break
    return len(poly)


with open("i5.txt") as f:
    mt = f.read().strip()
    master = list(mt)

# part 1
print(reduce(master))

# part 2
lengths = {l:remove_reduce(l, mt) for l in string.ascii_lowercase}
minlength = min(lengths, key=lambda e: lengths[e])
print(lengths[minlength])
