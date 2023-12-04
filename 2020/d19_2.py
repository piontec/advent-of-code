import re
from typing import List, Dict

regex_str = re.compile("\"\w+\"")


def build_regexp(rules: Dict[int, str], num: int, rules_cache: Dict[int, str]) -> str:
    if num in rules_cache.keys():
        return rules_cache[num]

    rule: str = rules[num]

    if regex_str.match(rule):
        res = rule.strip('"')
        rules_cache[num] = res
        return res

    or_ind = rule.find("|")
    if or_ind >= 0:
        left, right = rule.split("|")
        left_part = ""
        for rule_no in left.strip().split(" "):
            left_part += build_regexp(rules, int(rule_no), rules_cache)
        right_part = ""
        for rule_no in right.strip().split(" "):
            right_part += build_regexp(rules, int(rule_no), rules_cache)
        res = f"(({left_part})|({right_part}))"
        rules_cache[num] = res
        return res
    else:
        res = ""
        for rule_no in rule.split(" "):
            res += build_regexp(rules, int(rule_no), rules_cache)
        res = "(" + res + ")"
        rules_cache[num] = res
        return res


def find_matches(lines: List[str], rule_no: int) -> int:
    try:
        ind = lines.index("")
    except ValueError:
        ind = lines.index("\n")
    rule_lines = lines[:ind]
    rules_dict = {int(line.split(":")[0]): line.split(":")[1].strip() for line in rule_lines}

    rules_cache: Dict[int, str] = {}
    # patch rules
    # 8: 42 | 42 8
    rule_42 = build_regexp(rules_dict, 42, rules_cache)
    rules_cache[8] = "(" + rule_42 + "+)"
    # 11: 42 31 | 42 11 31
    rule_31 = build_regexp(rules_dict, 31, rules_cache)
    rules_cache[11] = f"(({rule_42}{rule_31})" \
                      + f"|({rule_42}" + "{2}" + f"{rule_31}" + "{2})" \
                      + f"|({rule_42}" + "{3}" + f"{rule_31}" + "{3})" \
                      + f"|({rule_42}" + "{4}" + f"{rule_31}" + "{4})" \
                      + f"|({rule_42}" + "{5}" + f"{rule_31}" + "{5})" \
                      + f"|({rule_42}" + "{6}" + f"{rule_31}" + "{6})" \
                      + f"|({rule_42}" + "{7}" + f"{rule_31}" + "{7}))"

    regexp_str = build_regexp(rules_dict, rule_no, rules_cache)

    regex = re.compile(regexp_str)
    counter = 0
    for line in lines[ind + 1:]:
        line = line.strip()
        match = regex.fullmatch(line)
        if match:
            counter += 1
    return counter


test1 = """42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
"""

res1 = find_matches(test1.splitlines(), 0)
assert res1 == 12

with open("i19.txt", "r") as f:
    lines = f.readlines()
res = find_matches(lines, 0)
print(res)
