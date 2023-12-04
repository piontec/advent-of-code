import numpy as np
from bitarray import bitarray
from bitarray.util import ba2int


def main() -> None:
    with open("i3.txt", "r") as i:
        lines = i.readlines()
    run(lines)


def reduce(arr: np.ndarray, to_keep: int) -> int:
    ind = 0
    while arr.shape[0] > 1:
        starts_with_one = np.count_nonzero(arr[:, ind])
        if to_keep:
            match = int(starts_with_one >= arr.shape[0] - starts_with_one)
        else:
            match = int(starts_with_one < arr.shape[0] - starts_with_one)
        arr = np.delete(arr, np.where(arr[:, ind] != match), 0)
        ind += 1
    res = ba2int(bitarray([bool(n) for n in arr[0]]))
    return res


def run(lines: list[str]) -> None:
    nums = [[int(n) for n in line.strip()] for line in lines]
    oxy_arr = np.array(nums)
    co2_arr = oxy_arr.copy()
    oxygen = reduce(oxy_arr, 1)
    co2 = reduce(co2_arr, 0)
    print(oxygen * co2)


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
