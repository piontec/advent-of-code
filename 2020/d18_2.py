from typing import List, Callable


def find_most_nested_parenthesis(tokens: List[str]) -> (int, int, bool):
    start = 0
    for ind in range(len(tokens)):
        token = tokens[ind]
        if token.startswith("("):
            start = ind
        if token.endswith(")"):
            return start, ind, True
    return -1, -1, False


def reduce(tokens: List[str], symbol: str, fun: Callable[[int, int], int]) -> List[str]:
    while True:
        for ind in range(len(tokens)):
            if tokens[ind] == symbol:
                val = fun(int(tokens[ind - 1]), int(tokens[ind + 1]))
                tokens = tokens[:ind - 1] + [str(val)] + tokens[ind + 2:]
                break
        else:
            return tokens


def parse_nodes(tokens: List[str]) -> int:
    while True:
        start, end, found = find_most_nested_parenthesis(tokens)
        if not found:
            break
        last_left_parenthesis = tokens[start].rfind("(")
        first_right_parenthesis = tokens[end].find(")")
        add_right_parenthesis = len(tokens[end]) - first_right_parenthesis - 1
        tokens[start] = tokens[start][last_left_parenthesis + 1:]
        tokens[end] = tokens[end][:first_right_parenthesis]
        val = parse_nodes(tokens[start:end + 1])
        subst = ""
        for i in range(last_left_parenthesis):
            subst += "("
        subst += str(val)
        for i in range(add_right_parenthesis):
            subst += ")"
        tokens = tokens[:start] + [subst] + tokens[end + 1:]
    if len(tokens) == 1:
        return int(tokens[0])
    tokens = reduce(tokens, "+", lambda x1, x2: x1 + x2)
    tokens = reduce(tokens, "*", lambda x1, x2: x1 * x2)
    assert len(tokens) == 1
    return int(tokens[0])


def parse(expr: str) -> int:
    nodes = expr.strip().split(" ")
    return parse_nodes(nodes)


def sum_all(lines: List[str]) -> int:
    return sum(parse(line) for line in lines)


assert parse("1 + (2 * 3) + (4 * (5 + 6))") == 51
# assert parse("2 * 3 + (4 * 5)") == 46
# assert parse("5 + (8 * 3 + 9 + 3 * 4 * 3)") == 1445
# assert parse("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))") == 669060
# assert parse("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2") == 23340

with open("i18.txt", "r") as f:
    lines = f.readlines()
res = sum_all(lines)
print(res)
