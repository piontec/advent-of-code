import re
from typing import List, Dict

rule_re = re.compile("(\w+ \w+) bags contain (.+).")
count_rule = re.compile("(\d+) (\w+ \w+) bags?")


def build_rules(rules_txt: List[str]) -> Dict[str, Dict[str, int]]:
    rules: Dict[str, Dict[str, int]] = {}
    for line in rules_txt:
        ml = rule_re.match(line)
        left_color = ml.group(1)
        rules[left_color] = {}
        contains_txt = ml.group(2).split(",")
        for c in contains_txt:
            if c == "no other bags":
                continue
            mc = count_rule.match(c.strip())
            color = mc.group(2)
            count = int(mc.group(1))
            rules[left_color][color] = count
    return rules


def bag_count(rules: Dict[str, Dict[str, int]], color: str) -> int:
    inside = rules[color]
    if len(inside) == 0:
        return 0
    bag_sum = 0
    for color, count in inside.items():
        bag_sum += count * (bag_count(rules, color) + 1)
    return bag_sum


def run(rules_txt: List[str], bag_color: str) -> int:
    rules = build_rules(rules_txt)
    return bag_count(rules, bag_color)


test1 = """shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
"""

res = run(test1.splitlines(), "shiny gold")
assert res == 126

with open("i7.txt", "r") as f:
    lines = f.readlines()
print(run(lines, "shiny gold"))
