import math
from typing import Tuple

from bitarray import bitarray


def main() -> None:
    with open("i16.txt", "r") as i:
        lines = i.readlines()
    res = run(lines[0].strip())
    print(res)


def hex_str_to_bitarray(line: str) -> bitarray:
    symbols = {
        "0": bitarray("0000"),
        "1": bitarray("0001"),
        "2": bitarray("0010"),
        "3": bitarray("0011"),
        "4": bitarray("0100"),
        "5": bitarray("0101"),
        "6": bitarray("0110"),
        "7": bitarray("0111"),
        "8": bitarray("1000"),
        "9": bitarray("1001"),
        "A": bitarray("1010"),
        "B": bitarray("1011"),
        "C": bitarray("1100"),
        "D": bitarray("1101"),
        "E": bitarray("1110"),
        "F": bitarray("1111"),
    }
    res = bitarray()
    res.encode(symbols, line)
    return res


def bitarray_to_int(ba: bitarray) -> int:
    res: int = 0
    power: int = 0
    for i in range(0, len(ba)):
        res += ba[len(ba) - 1 - i] * 2 ** power
        power += 1
    return res


def decode_literal_value(val: bitarray) -> (int, int):
    bits = val.copy()
    num_arr = bitarray()
    processed_bits = 0
    while bits[0]:
        num_arr += bits[1:5]
        del bits[:5]
        processed_bits += 5
    num_arr += bits[1:5]
    processed_bits += 5
    res = bitarray_to_int(num_arr)
    return res, processed_bits


def parse_packets(bits: bitarray, start_index: int, packets_count: int = 0, packets_length: int = 0) -> Tuple[
                  int, list[int, int]]:
    processed_packets = 0
    index = start_index
    values: list[int] = []
    while not (
            (processed_packets > 0 and processed_packets == packets_count) or (index > 0 and index == packets_length)):
        if len(bits[index:]) <= 7 and not (bits[index:].any()):
            # i hope it's padding
            break
        version = bitarray_to_int(bits[index:index + 3])
        type_id = bitarray_to_int(bits[index + 3:index + 6])
        index += 6
        processed_packets += 1
        if type_id == 4:
            val, fwd = decode_literal_value(bits[index:])
            index += fwd
            values.append(val)
        else:
            mode = bits[index]
            index += 1
            if mode == 0:
                total_len = bitarray_to_int(bits[index:index + 15])
                index += 15
                ind_add, vals = parse_packets(bits[index:], 0, packets_length=total_len)
            else:
                packet_count = bitarray_to_int(bits[index:index + 11])
                index += 11
                ind_add, vals = parse_packets(bits[index:], 0, packets_count=packet_count)
            index += ind_add
            match type_id:
                case 0:
                    values.append(sum(vals))
                case 1:
                    values.append(math.prod(vals))
                case 2:
                    values.append(min(vals))
                case 3:
                    values.append(max(vals))
                case 5:
                    values.append(1 if vals[0] > vals[1] else 0)
                case 6:
                    values.append(1 if vals[0] < vals[1] else 0)
                case 7:
                    values.append(1 if vals[0] == vals[1] else 0)
                case _:
                    raise Exception("WTF?")

    return index, values


def run(line: str) -> int:
    bits = hex_str_to_bitarray(line)
    ind, values = parse_packets(bits, 0, packets_count=1)
    assert len(values) == 1
    return values[0]


def test() -> None:
    # assert run("C200B40A82") == 3
    # assert run("04005AC33890") == 54
    # assert run("880086C3E88112") == 7
    # assert run("CE00C43D881120") == 9
    # assert run("D8005AC2A8F0") == 1
    # assert run("F600BC2D8F") == 0
    # assert run("9C005AC2F8F0") == 0
    assert run("9C0141080250320F1802104A08") == 1


if __name__ == "__main__":
    test()
    main()
