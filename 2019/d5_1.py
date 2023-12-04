from typing import List


def parse_instruction(instruction: int) -> (int, List[int]):
    opcode = instruction % 100
    args = 3 if (opcode == 1 or opcode == 2) else 1
    modes: List[int] = []
    mds = instruction // 100
    for i in range(args):
        mode = mds % 10
        mds //= 10
        modes.append(mode)
    return (opcode, modes)


def get_argument(code: List[int], ip: int, arg_modes: List[int], offset: int) -> int:
    # position mode
    if arg_modes[offset] == 0:
        return code[code[ip + 1 + offset]]
    # immediate mode
    if arg_modes[offset] == 1:
        return code[ip + 1 + offset]
    raise AssertionError(str.format("Unknown arg mode: {}", arg_modes[offset]))


def run(strcode: str, inputs: List[int]) -> List[int]:
    code = [int(x) for x in str.split(strcode, ",")]
    outputs: List[int] = []
    ip = 0
    input_index = 0
    while code[ip] != 99:
        opcode, modes = parse_instruction(code[ip])
        if opcode == 1:
            assert modes[2] == 0
            code[code[ip + 3]] = get_argument(code, ip, modes, 0) + get_argument(code, ip, modes, 1)
            ip += 4
        elif opcode == 2:
            assert modes[2] == 0
            code[code[ip + 3]] = get_argument(code, ip, modes, 0) * get_argument(code, ip, modes, 1)
            ip += 4
        elif opcode == 3:
            assert modes[0] == 0
            code[code[ip + 1]] = inputs[input_index]
            input_index += 1
            ip += 2
        elif opcode == 4:
            if modes[0] == 0:
                outputs.append(code[code[ip + 1]])
            elif modes[0] == 1:
                outputs.append(code[ip + 1])
            ip += 2
        else:
            print(str.format("ERROR: unknown op code: {}", code[ip]))
            return
    return outputs


assert run("1002,4,3,4,33", []) == []
with open("i5.txt") as f:
    lines = f.readlines()
res = run(lines[0], [1])
codes = [x for x in res if x != 0]
print(str.format("Part 1: {}", codes))
