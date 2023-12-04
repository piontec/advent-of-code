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


def run(rules_txt: List[str], bag_color: str) -> int:
    rules = build_rules(rules_txt)
    matched_colors: List[str] = []
    to_check = [bag_color]
    while len(to_check) > 0:
        searched_color = to_check.pop()
        for left_color, rights in rules.items():
            if searched_color in rights.keys():
                if left_color not in to_check and left_color not in matched_colors:
                    to_check.append(left_color)
                if left_color not in matched_colors:
                    matched_colors.append(left_color)
    return len(matched_colors)


test1 = """light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
"""

assert run(test1.splitlines(), "shiny gold") == 4

with open("i7.txt", "r") as f:
    lines = f.readlines()
print(run(lines, "shiny gold"))
