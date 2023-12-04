import numpy as np
from bitarray import bitarray
from bitarray.util import ba2int


def main() -> None:
    with open("i3.txt", "r") as i:
        lines = i.readlines()
    run(lines)


def run(lines: list[str]) -> None:
    nums = [[int(n) for n in line.strip()] for line in lines]
    arr = np.array(nums)
    limit = arr.shape[0] // 2
    ones = np.count_nonzero(arr, axis=0)
    gamma_arr = bitarray([n > limit for n in ones])
    epsilon_arr = ~gamma_arr
    gamma = ba2int(gamma_arr)
    epsilon = ba2int(epsilon_arr)
    print(gamma * epsilon)


def test() -> None:
    txt = """00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"""
    lines = txt.splitlines()
    run(lines)


if __name__ == "__main__":
    test()
    main()
