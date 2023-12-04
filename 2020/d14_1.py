from typing import List, Dict, Tuple

import regex

mask_re = regex.compile("mask = ([X10]+)")
mem_re = regex.compile("mem\[(\d+)\] = (\d+)")

bits = 36
max_val = 0xFFFFFFFFF


def get_masks(mask: str) -> Tuple[int, int]:
    or_mask = 0
    and_mask = max_val
    for bit_index in range(len(mask)):
        bit = mask[bit_index]
        if bit == "0":
            and_mask &= (max_val - 1)
        elif bit == "1":
            or_mask |= 1
        if bit_index == len(mask) - 1:
            break
        and_mask <<= 1
        and_mask |= 1
        or_mask <<= 1
    return or_mask, and_mask


def run_prog(lines: List[str]) -> int:
    or_mask: int = 0
    and_mask: int = 0
    mem: Dict[int, int] = {}
    for line in lines:
        match_mask = mask_re.match(line)
        if match_mask:
            or_mask, and_mask = get_masks(match_mask.group(1))
            continue
        match_mem = mem_re.match(line)
        if not match_mem:
            raise Exception("no RE matches")
        value = int(match_mem.group(2))
        value = (value | or_mask) & and_mask
        addr = int(match_mem.group(1))
        mem[addr] = value
    return sum(mem.values())


test1 = """mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
"""
res = run_prog(test1.splitlines())
assert res == 165

with open("i14.txt", "r") as f:
    lines = f.readlines()
res = run_prog(lines)
print(res)
