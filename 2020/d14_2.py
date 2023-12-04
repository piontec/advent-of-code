from typing import List, Dict

import regex

mask_re = regex.compile("mask = ([X10]+)")
mem_re = regex.compile("mem\[(\d+)\] = (\d+)")

bits = 36
max_val = 0xFFFFFFFFF


def get_addrs(addr: int, mask: str) -> List[int]:
    addr_str = list("{:036b}".format(addr))
    for bit_index in range(len(mask)):
        if mask[bit_index] in ["1", "X"]:
            addr_str[bit_index] = mask[bit_index]

    addrs = [addr_str]
    while True:
        a = addrs[0]
        for i in range(len(a)):
            if a[i] == "X":
                addrs.pop(0)
                with_0 = a.copy()
                with_0[i] = "0"
                with_1 = a.copy()
                with_1[i] = "1"
                addrs.extend([with_0, with_1])
                break
        else:
            break
    res = [int(x, 2) for x in ["".join(l) for l in addrs]]
    return res


def run_prog(lines: List[str]) -> int:
    mem: Dict[int, int] = {}
    mask: str = ""
    for line in lines:
        match_mask = mask_re.match(line)
        if match_mask:
            mask = match_mask.group(1)
            continue
        match_mem = mem_re.match(line)
        if not match_mem:
            raise Exception("no RE matches")
        value = int(match_mem.group(2))
        addresses = get_addrs(int(match_mem.group(1)), mask)
        for addr in addresses:
            mem[addr] = value
    return sum(mem.values())


test1 = """mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
"""
res = run_prog(test1.splitlines())
assert res == 208

with open("i14.txt", "r") as f:
    lines = f.readlines()
res = run_prog(lines)
print(res)
