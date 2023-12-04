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


def parse_packets(bits: bitarray, start_index: int, packets_count: int = 0, packets_length: int = 0) -> Tuple[int, int]:
    processed_packets = 0
    index = start_index
    ver_sum = 0
    while not (
            (processed_packets > 0 and processed_packets == packets_count) or (index > 0 and index == packets_length)):
        if len(bits[index:]) <= 7 and not(bits[index:].any()):
            # i hope it's padding
            break
        version = bitarray_to_int(bits[index:index + 3])
        ver_sum += version
        type_id = bitarray_to_int(bits[index + 3:index + 6])
        index += 6
        packets_count += 1
        if type_id == 4:
            val, fwd = decode_literal_value(bits[index:])
            index += fwd
        else:
            mode = bits[index]
            index += 1
            if mode == 0:
                total_len = bitarray_to_int(bits[index:index + 15])
                index += 15
                ind_add, sum_add = parse_packets(bits[index:], 0, packets_length=total_len)
            else:
                packet_count = bitarray_to_int(bits[index:index + 11])
                index += 11
                ind_add, sum_add = parse_packets(bits[index:], 0, packets_count=packet_count)
            index += ind_add
            ver_sum += sum_add
    return index, ver_sum


def run(line: str) -> int:
    bits = hex_str_to_bitarray(line)
    ind, ver_sum = parse_packets(bits, 0, packets_count=1)
    return ver_sum


def test() -> None:
    # test_val = hex_str_to_bitarray("D2FE28")[6:]
    # assert decode_literal_value(test_val) == (2021, 18)
    test_val = hex_str_to_bitarray("38006F45291200")
    parse_packets(test_val, 0, packets_count=1)
    assert run("8A004A801A8002F478") == 16
    assert run("620080001611562C8802118E34") == 12
    assert run("C0015000016115A2E0802F182340") == 23
    assert run("A0016C880162017C3686B18A3D4780") == 31


if __name__ == "__main__":
    test()
    main()
