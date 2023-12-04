from collections import deque
from typing import Tuple


def main() -> None:
    with open("i10.txt", "r") as i:
        lines = i.readlines()
    res = run(lines)
    print(res)


def opener(c: str) -> bool:
    assert len(c) == 1
    return c == "(" or c == "[" or c == "<" or c == "{"


def get_error_score(c: str) -> int:
    assert len(c) == 1
    match c:
        case ")":
            return 3
        case "]":
            return 57
        case "}":
            return 1197
        case ">":
            return 25137
        case _:
            raise Exception("Unknown char")


def get_complement(c: str) -> str:
    assert len(c) == 1
    match c:
        case "(":
            return ")"
        case "[":
            return "]"
        case "{":
            return "}"
        case "<":
            return ">"
        case _:
            raise Exception("Unknown char")


def parse(line: str) -> Tuple[bool, int]:
    """Returns tuple (complete, error_score)"""
    stack = deque()
    for char in line:
        if opener(char):
            stack.append(char)
        else:
            if char == get_complement(stack[len(stack) - 1]):
                stack.pop()
            else:
                return True, get_error_score(char)
    if len(stack) > 0:
        return False, 0


def run(lines: list[str]) -> int:
    total = 0
    for line in lines:
        complete, score = parse(line.strip())
        if not complete:
            continue
        total += score
    return total


def test() -> None:
    txt = """[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"""
    assert run(txt.splitlines()) == 26397


if __name__ == "__main__":
    test()
    main()
