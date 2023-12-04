def run(strcode: str, ovr1: int, ovr2: int):
    code = [int(x) for x in str.split(strcode, ",")]
    code[1] = ovr1
    code[2] = ovr2
    current = 0
    while code[current] != 99:
        if code[current] == 1:
            code[code[current + 3]] = code[code[current + 1]] + code[code[current + 2]]
        elif code[current] == 2:
            code[code[current + 3]] = code[code[current + 1]] * code[code[current + 2]]
        else:
            print(str.format("ERROR: unknown op code: {}", code[current]))
            return
        current += 4
    return code

def part2(strcode: str, wanted: int):
    for ovr1 in range(99):
        for ovr2 in range(99):
            if run(strcode, ovr1, ovr2)[0] == wanted:
                return 100 * ovr1 + ovr2

assert run("1,9,10,3,2,3,11,0,99,30,40,50", 9, 10)[0] == 3500
assert run("1,1,1,4,99,5,6,0,99", 1, 1)[0] == 30
with open("i2.txt") as f:
    lines = f.readlines()
res = run(lines[0], 12, 2)
print(str.format("Part 1: {}", res[0]))
res2 = part2(lines[0], 19690720)
print(str.format("Part 2: {}", res2))