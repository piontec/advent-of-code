import re
from typing import List

num = re.compile("\d+")


def find_end_parenthesis(tokens: List[str]) -> int:
    nesting = 0
    for ind in range(len(tokens)):
        token = tokens[ind]
        while token.startswith("("):
            nesting += 1
            token = token[1:]
        while token.endswith(")"):
            nesting -= 1
            token = token[:-1]
            if nesting == 0:
                return ind
    raise Exception("not found")


def parse_nodes(tokens: List[str], acc: int, op: str) -> int:
    token = tokens[0]
    left = 0
    if token.startswith("("):
        end_parenthesis_index = find_end_parenthesis(tokens)
        tokens[0] = token[1:]
        tokens[end_parenthesis_index] = tokens[end_parenthesis_index][:-1]
        left = parse_nodes(tokens[0:end_parenthesis_index + 1], acc, "=")
        tokens = [str(left)] + tokens[end_parenthesis_index + 1:]
    else:
        left = int(token)
    if op == "=":
        acc = left
    elif op == "+":
        acc += left
    elif op == "*":
        acc *= left
    if len(tokens) <= 2:
        return acc
    else:
        left_tokens = tokens[2:]
        op = tokens[1]
        return parse_nodes(left_tokens, acc, op)


def parse(expr: str) -> int:
    nodes = expr.strip().split(" ")
    return parse_nodes(nodes, 0, "=")


def sum_all(lines: List[str]) -> int:
    return sum(parse(line) for line in lines)


# assert parse("2") == 2
# assert parse("2 + 1") == 3
# assert parse("1 + 2 * 3") == 9
# assert parse("(1 + 2) * 3") == 9
# assert parse("2 * 3 + (4 * 5)") == 26
# assert parse("5 + (8 * 3 + 9 + 3 * 4 * 3)") == 437
# assert parse("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))") == 12240
assert parse("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2") == 13632

with open("i18.txt", "r") as f:
    lines = f.readlines()
res = sum_all(lines)
print(res)
