from typing import Tuple


def main() -> None:
    with open("i8.txt", "r") as i:
        lines = i.readlines()
    res = run(lines)
    print(res)


def run(lines: list[str]) -> int:
    easy_digits = 0
    for line in lines:
        signal_patterns_txt, output_val_txt = line.strip().split("|")
        output_vals = output_val_txt.strip().split(" ")
        easy_digits += sum([1 if len(d) in [2, 3, 4, 7] else 0 for d in output_vals])
    return easy_digits


def test() -> None:
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
    assert run(txt.splitlines()) == 26


if __name__ == "__main__":
    test()
    main()
