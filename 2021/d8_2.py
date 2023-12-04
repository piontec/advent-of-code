def main() -> None:
    with open("i8.txt", "r") as i:
        lines = i.readlines()
    res = run(lines)
    print(res)


def run(lines: list[str]) -> int:
    num_sum = 0
    for line in lines:
        signal_patterns_txt, output_val_txt = line.strip().split("|")
        signal_patterns = signal_patterns_txt.strip().split(" ")
        output_vals = output_val_txt.strip().split(" ")
        seg5 = []
        seg6 = []
        pattern_to_digit: dict[frozenset, int] = {}
        for pat in signal_patterns:
            match len(pat):
                case 2:
                    dig1 = frozenset(pat)
                    pattern_to_digit[dig1] = 1
                case 3:
                    dig7 = frozenset(pat)
                    pattern_to_digit[dig7] = 7
                case 4:
                    dig4 = frozenset(pat)
                    pattern_to_digit[dig4] = 4
                case 5:
                    seg5.append(frozenset(pat))
                case 6:
                    seg6.append(frozenset(pat))
                case 7:
                    dig8 = frozenset(pat)
                    pattern_to_digit[dig8] = 8
        top_bar = dig7.difference(dig1)
        horizontals = seg5[0].intersection(seg5[1]).intersection(seg5[2])
        mid_bar = dig4.intersection(horizontals)
        bottom_bar = horizontals.difference(top_bar | mid_bar)
        # if 1 and one of [6, 9, 0] have only 1 common bar, it's 6, else 9 or 0
        for seg in seg6:
            if len(seg.intersection(dig1)) == 1:
                dig6 = seg
                pattern_to_digit[dig6] = 6
            elif len(seg.intersection(horizontals)) == 2:
                dig0 = seg
                pattern_to_digit[dig0] = 0
            else:
                dig9 = seg
                pattern_to_digit[dig9] = 9
        bottom_right = dig1.intersection(dig6)
        top_right = dig1.difference(bottom_right)
        top_left = dig4.difference(mid_bar | top_right | bottom_right)
        bottom_left = dig8.difference(dig9)
        dig2 = horizontals | top_right | bottom_left
        pattern_to_digit[dig2] = 2
        dig3 = horizontals | top_right | bottom_right
        pattern_to_digit[dig3] = 3
        dig5 = horizontals | top_left | bottom_right
        pattern_to_digit[dig5] = 5

        mult = 1
        num = 0
        output_vals.reverse()
        for val in output_vals:
            digit = pattern_to_digit[frozenset(val)]
            num += digit * mult
            mult *= 10
        num_sum += num
    return num_sum


def test() -> None:
    txt = "ecfdbg decfba aegd fdcag fagecd gd gcafb efdac cbgeafd dfg | bgacf afdebc fceda cabfg"
    assert run([txt]) == 2652

    txt = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
    assert run([txt]) == 5353

    txt = """be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"""
    assert run(txt.splitlines()) == 61229


if __name__ == "__main__":
    test()
    main()
