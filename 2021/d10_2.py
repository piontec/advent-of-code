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


def get_complete_score(c: str) -> int:
    assert len(c) == 1
    match c:
        case ")":
            return 1
        case "]":
            return 2
        case "}":
            return 3
        case ">":
            return 4
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
                return True, 0
    if len(stack) > 0:
        score = 0
        stack.reverse()
        while len(stack) > 0:
            char = stack.popleft()
            score = (score * 5) + get_complete_score(get_complement(char))
        return False, score


def run(lines: list[str]) -> int:
    scores: list[int] = []
    for line in lines:
        complete, score = parse(line.strip())
        if not complete:
            scores.append(score)
    scores.sort()
    return scores[(len(scores) - 1) // 2]


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
    assert run(txt.splitlines()) == 288957


if __name__ == "__main__":
    test()
    main()
